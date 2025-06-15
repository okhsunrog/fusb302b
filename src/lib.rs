#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;

use device_driver::{AsyncRegisterInterface, AsyncBufferInterface};
use embedded_hal_async::i2c::I2c;
use thiserror::Error;
use crate::field_sets::DeviceId;
use usbpd_traits::{Driver as SinkDriver, DriverRxError, DriverTxError};
use embassy_time::{Duration, Timer};

device_driver::create_device!(device_name: FusbLowLevel, manifest: "device.yaml");
pub const FUSB302B_I2C_ADDRESS: u8 = 0x22;

#[derive(Debug, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FusbError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    // #[error("Invalid voltage: {0}mV for setting")]
    // InvalidVoltage(u16),
    // #[error("Invalid current: {0}mA for setting")]
    // InvalidCurrent(u16),
    #[error("Feature or specific mode not supported/implemented: {0}")]
    NotSupported(&'static str),
    #[error("Data length exceeds buffer size")]
    LenExceedsBuffer,
}

pub struct DeviceInterface<I2CBus> {
    i2c: I2CBus,
}

impl<I> DeviceInterface<I> {
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }
}

impl<I, E> AsyncRegisterInterface for DeviceInterface<I>
where
    I: I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;

    type Error = FusbError<E>;

    async fn read_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(FUSB302B_I2C_ADDRESS, &[address], data)
            .await
            .map_err(FusbError::I2c)
    }

    async fn write_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 5];
        if (1 + data.len()) > buffer.len() {
            return Err(FusbError::LenExceedsBuffer);
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c
            .write(FUSB302B_I2C_ADDRESS, &buffer[..1 + data.len()])
            .await
            .map_err(FusbError::I2c)
    }
}

pub struct Fusb302b<
    I2CImpl: AsyncRegisterInterface<AddressType = u8, Error = FusbError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as AsyncRegisterInterface>::Error,
> {
    pub ll: FusbLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

impl<I2CBus, E> Fusb302b<DeviceInterface<I2CBus>, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: FusbLowLevel::new(DeviceInterface::new(i2c)),
            _marker: core::marker::PhantomData,
        }
    }

    pub async fn get_device_info(&mut self) -> Result<DeviceId, FusbError<E>> {
        self.ll.device_id().read_async().await
    }
}


impl<I2CBus, E> SinkDriver for Fusb302b<DeviceInterface<I2CBus>, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    // Our FUSB302B driver DOES have automatic GoodCRC capability!
    const HAS_AUTO_GOOD_CRC: bool = true;

    async fn wait_for_vbus(&self) {
        // For now assume it's always present as on my dev board vbus isn't connected
    }

    async fn transmit_hard_reset(&mut self) -> Result<(), DriverTxError> {
        self.ll.control_3().modify_async(|r| r.set_send_hard_reset(true)).await
            .map_err(|_| DriverTxError::Discarded)?;

        // Wait for Hard Reset to be sent, with a timeout
        let deadline = Timer::after(Duration::from_millis(5));
        loop {
            let irqa = self.ll.interrupta().read_async().await.map_err(|_| DriverTxError::Discarded)?;
            if irqa.i_hardsent() {
                return Ok(());
            }
            if deadline.expired() {
                return Err(DriverTxError::Discarded);
            }
            Timer::after_millis(1).await;
        }
    }

    async fn transmit(&mut self, data: &[u8]) -> Result<(), DriverTxError> {
        // Construct the full packet for the FUSB302B FIFO
        let mut fifo_buffer = [0u8; 40];
        let mut pos = 0;

        // Preamble + PACKSYM
        fifo_buffer[pos..pos+5].copy_from_slice(&[token::SOP1, token::SOP1, token::SOP1, token::SOP2, token::PACK_SYM | (data.len() as u8)]);
        pos += 5;
        
        // Header + Payload
        fifo_buffer[pos..pos+data.len()].copy_from_slice(data);
        pos += data.len();

        // Trailer + Control
        fifo_buffer[pos..pos+4].copy_from_slice(&[token::JAM_CRC, token::EOP, token::TX_OFF, token::TX_ON]);
        pos += 4;
        
        // Write the whole packet to the FIFO
        self.ll.fifo().write_all(&fifo_buffer[..pos]).await.map_err(|_| DriverTxError::Discarded)?;

        // Wait for confirmation (TXSENT) or error
        let deadline = Timer::after(Duration::from_millis(5));
        loop {
            let irqa = self.ll.interrupta().read_async().await.map_err(|_| DriverTxError::Discarded)?;
            if irqa.i_txsent() {
                return Ok(());
            }
            if irqa.i_hardrst() {
                return Err(DriverTxError::HardReset);
            }
            if self.ll.interrupt().read_async().await.map_err(|_| DriverTxError::Discarded)?.i_collision() {
                return Err(DriverTxError::Discarded);
            }
            if deadline.expired() {
                return Err(DriverTxError::Discarded);
            }
            Timer::after_millis(1).await;
        }
    }

    async fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, DriverRxError> {
        // Wait for a relevant interrupt
        let deadline = Timer::after(Duration::from_millis(20)); // tReceive timeout
        loop {
            if self.ll.interrupta().read_async().await.map_err(|_| DriverRxError::Discarded)?.i_hardrst() {
                return Err(DriverRxError::HardReset);
            }
            if self.ll.interrupt().read_async().await.map_err(|_| DriverRxError::Discarded)?.i_crc_chk() {
                break; // Message is ready
            }
            if deadline.expired() {
                return Err(DriverRxError::Discarded);
            }
            Timer::after_millis(1).await;
        }

        // Read the SOP* token first
        let mut token_buf = [0u8; 1];
        self.ll.fifo().read(&mut token_buf).await.map_err(|_| DriverRxError::Discarded)?;

        // Check it's a valid SOP* packet token
        if (token_buf[0] & 0b1110_0000) != 0b1110_0000 {
            self.ll.control_1().modify(|r| r.set_rx_flush(true)).await.ok();
            return Err(DriverRxError::Discarded);
        }

        // Read header
        let mut header_buf = [0u8; 2];
        self.ll.fifo().read_exact(&mut header_buf).await.map_err(|_| DriverRxError::Discarded)?;
        let header = u16::from_le_bytes(header_buf);
        let num_data_objects = ((header >> 12) & 0x7) as usize;
        let payload_len = num_data_objects * 4;
        let total_len = 2 + payload_len;

        if total_len > buffer.len() {
            self.ll.control_1().modify(|r| r.set_rx_flush(true)).await.ok();
            return Err(DriverRxError::Discarded);
        }

        buffer[0..2].copy_from_slice(&header_buf);
        if payload_len > 0 {
            self.ll.fifo().read_exact(&mut buffer[2..total_len]).await.map_err(|_| DriverRxError::Discarded)?;
        }
        
        // Read out the 4-byte CRC to clear the rest of the message from the FIFO
        let mut crc_buf = [0u8; 4];
        self.ll.fifo().read_exact(&mut crc_buf).await.map_err(|_| DriverRxError::Discarded)?;
        
        Ok(total_len)
    }
}