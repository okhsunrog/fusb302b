#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;

mod token {
    pub const SOP1: u8 = 0x12;
    pub const SOP2: u8 = 0x13;
    pub const PACK_SYM: u8 = 0x80;
    pub const JAM_CRC: u8 = 0xFF;
    pub const EOP: u8 = 0x14;
    pub const TX_OFF: u8 = 0xFE;
    pub const TX_ON: u8 = 0xA1;
}

use device_driver::{AsyncBufferInterface, AsyncRegisterInterface, BufferInterfaceError};
use embedded_hal_async::i2c::I2c;
use thiserror::Error;

use crate::field_sets::{
    Control0, Control1, DeviceId, Mask, Maska, Maskb, Power, Reset, Switches0, Switches1,
};
use embassy_time::{Duration, Instant, Timer};
use usbpd_traits::{Driver as SinkDriver, DriverRxError, DriverTxError};

device_driver::create_device!(device_name: FusbLowLevel, manifest: "device.yaml");
pub const FUSB302B_I2C_ADDRESS: u8 = 0x22;

#[derive(Debug, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FusbError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Data length exceeds internal buffer size for I2C transaction")]
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

impl<I, E> BufferInterfaceError for DeviceInterface<I>
where
    I: I2c<Error = E>,
    E: core::fmt::Debug,
{
    type Error = FusbError<E>;
}

impl<I, E> AsyncBufferInterface for DeviceInterface<I>
where
    I: I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;

    async fn write(&mut self, address: u8, buf: &[u8]) -> Result<usize, Self::Error> {
        // This implementation detail is simplified; real-world might need heapless vector or DMA.
        let mut buffer = [0u8; 64];
        if (1 + buf.len()) > buffer.len() {
            return Err(FusbError::LenExceedsBuffer);
        }
        buffer[0] = address;
        buffer[1..1 + buf.len()].copy_from_slice(buf);
        self.i2c
            .write(FUSB302B_I2C_ADDRESS, &buffer[..1 + buf.len()])
            .await
            .map_err(FusbError::I2c)?;
        Ok(buf.len())
    }

    async fn read(&mut self, address: u8, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.i2c
            .write_read(FUSB302B_I2C_ADDRESS, &[address], buf)
            .await
            .map_err(FusbError::I2c)?;
        Ok(buf.len())
    }

    async fn flush(&mut self, _address: u8) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct Fusb302b<I2CBus, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub ll: FusbLowLevel<DeviceInterface<I2CBus>>,
    _marker: core::marker::PhantomData<E>,
}

impl<I2CBus, E> Fusb302b<I2CBus, E>
where
    I2CBus: I2c<Error = E> + 'static,
    E: core::fmt::Debug,
{
    pub async fn new_and_init(i2c: I2CBus) -> Result<Self, FusbError<E>> {
        let mut driver = Self {
            ll: FusbLowLevel::new(DeviceInterface::new(i2c)),
            _marker: core::marker::PhantomData,
        };

        // Initialize Reset register
        driver
            .ll
            .reset()
            .write_async(|r| {
                r.set_sw_res(true);
                r.set_pd_reset(true);
            })
            .await?;

        Timer::after_millis(10).await;

        // Initialize Power register
        driver
            .ll
            .power()
            .write_async(|r| {
                r.set_pwr0_bandgap_and_wake_enable(true);
                r.set_pwr1_receiver_and_measure_refs_enable(true);
                r.set_pwr2_measure_block_power_enable(true);
            })
            .await?;

        // Initialize Switches0 register
        driver
            .ll
            .switches_0()
            .write_async(|r| {
                r.set_pdwn_1(true);
                r.set_pdwn_2(true);
                r.set_meas_cc_1(true);
            })
            .await?;

        // Initialize Switches1 register
        driver
            .ll
            .switches_1()
            .modify_async(|r| {
                r.set_auto_crc(true);
                r.set_txcc_1(true);
            })
            .await?;

        // Clear all interrupt masks
        driver.ll.mask().write_async(|r| *r = Mask::new()).await?;
        driver.ll.maska().write_async(|r| *r = Maska::new()).await?;
        driver.ll.maskb().write_async(|r| *r = Maskb::new()).await?;

        // Enable interrupts from the chip
        driver
            .ll
            .control_0()
            .modify_async(|r| {
                r.set_int_mask(false);
            })
            .await?;

        Ok(driver)
    }

    pub async fn get_device_info(&mut self) -> Result<DeviceId, FusbError<E>> {
        self.ll.device_id().read_async().await
    }
}

impl<I2CBus, E> SinkDriver for Fusb302b<I2CBus, E>
where
    I2CBus: I2c<Error = E> + 'static,
    E: core::fmt::Debug,
{
    const HAS_AUTO_GOOD_CRC: bool = true;

    async fn wait_for_vbus(&self) {}

    async fn transmit_hard_reset(&mut self) -> Result<(), DriverTxError> {
        self.ll
            .control_3()
            .modify_async(|r| r.set_send_hard_reset(true))
            .await
            .map_err(|_| DriverTxError::Discarded)?;

        let deadline = Instant::now() + Duration::from_millis(5);
        loop {
            let irqa = self
                .ll
                .interrupta()
                .read_async()
                .await
                .map_err(|_| DriverTxError::Discarded)?;
            if irqa.i_hardsent() {
                self.ll
                    .interrupta()
                    .modify_async(|r| r.set_i_hardsent(true))
                    .await
                    .ok();
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(DriverTxError::Discarded);
            }
            Timer::after_millis(1).await;
        }
    }

    async fn transmit(&mut self, data: &[u8]) -> Result<(), DriverTxError> {
        self.ll
            .power()
            .modify_async(|r| r.set_pwr3_internal_oscillator_enable(true))
            .await
            .map_err(|_| DriverTxError::Discarded)?;

        let fifo_result = async {
            let mut fifo_buffer = [0u8; 40];
            let mut pos = 0;
            let packet_byte_count = 2 + data.len();
            fifo_buffer[pos..pos + 5].copy_from_slice(&[
                token::SOP1,
                token::SOP1,
                token::SOP1,
                token::SOP2,
                token::PACK_SYM | (packet_byte_count as u8),
            ]);
            pos += 5;

            fifo_buffer[pos..pos + data.len()].copy_from_slice(data);
            pos += data.len();

            fifo_buffer[pos..pos + 4].copy_from_slice(&[
                token::JAM_CRC,
                token::EOP,
                token::TX_OFF,
                token::TX_ON,
            ]);
            pos += 4;
            self.ll.fifo().write_all_async(&fifo_buffer[..pos]).await
        }
        .await;

        if fifo_result.is_err() {
            self.ll
                .power()
                .modify_async(|r| r.set_pwr3_internal_oscillator_enable(false))
                .await
                .ok();
            return Err(DriverTxError::Discarded);
        }

        let deadline = Instant::now() + Duration::from_millis(5);
        let mut tx_result = Err(DriverTxError::Discarded);

        loop {
            let irqa = self
                .ll
                .interrupta()
                .read_async()
                .await
                .map_err(|_| DriverTxError::Discarded)?;
            if irqa.i_txsent() {
                self.ll
                    .interrupta()
                    .modify_async(|r| r.set_i_txsent(true))
                    .await
                    .ok();
                tx_result = Ok(());
                break;
            }
            if irqa.i_hardrst() {
                tx_result = Err(DriverTxError::HardReset);
                break;
            }
            if self
                .ll
                .interrupt()
                .read_async()
                .await
                .map_err(|_| DriverTxError::Discarded)?
                .i_collision()
            {
                self.ll
                    .interrupt()
                    .modify_async(|r| r.set_i_collision(true))
                    .await
                    .ok();
                tx_result = Err(DriverTxError::Discarded);
                break;
            }
            if Instant::now() >= deadline {
                break;
            }
            Timer::after_millis(1).await;
        }

        self.ll
            .power()
            .modify_async(|r| r.set_pwr3_internal_oscillator_enable(false))
            .await
            .ok();
        tx_result
    }

    async fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, DriverRxError> {
        let deadline = Instant::now() + Duration::from_millis(20);
        loop {
            let irqa = self
                .ll
                .interrupta()
                .read_async()
                .await
                .map_err(|_| DriverRxError::Discarded)?;
            if irqa.i_hardrst() {
                return Err(DriverRxError::HardReset);
            }
            let irq = self
                .ll
                .interrupt()
                .read_async()
                .await
                .map_err(|_| DriverRxError::Discarded)?;
            if irq.i_crc_chk() {
                self.ll
                    .interrupt()
                    .modify_async(|r| r.set_i_crc_chk(true))
                    .await
                    .ok();
                break;
            }
            if Instant::now() >= deadline {
                return Err(DriverRxError::Discarded);
            }
            Timer::after_millis(1).await;
        }

        let mut token_buf = [0u8; 1];
        self.ll
            .fifo()
            .read_async(&mut token_buf)
            .await
            .map_err(|_| DriverRxError::Discarded)?;

        if (token_buf[0] & 0b1110_0000) != 0b1110_0000 {
            self.ll
                .control_1()
                .modify_async(|r| r.set_rx_flush(true))
                .await
                .ok();
            return Err(DriverRxError::Discarded);
        }

        let mut header_buf = [0u8; 2];
        self.ll
            .fifo()
            .read_exact_async(&mut header_buf)
            .await
            .map_err(|_| DriverRxError::Discarded)?;

        let header = u16::from_le_bytes(header_buf);
        let num_data_objects = ((header >> 12) & 0x7) as usize;
        let payload_len = num_data_objects * 4;
        let total_len = 2 + payload_len;

        if total_len > buffer.len() {
            self.ll
                .control_1()
                .modify_async(|r| r.set_rx_flush(true))
                .await
                .ok();
            return Err(DriverRxError::Discarded);
        }

        buffer[0..2].copy_from_slice(&header_buf);
        if payload_len > 0 {
            self.ll
                .fifo()
                .read_exact_async(&mut buffer[2..total_len])
                .await
                .map_err(|_| DriverRxError::Discarded)?;
        }

        let mut crc_buf = [0u8; 4];
        self.ll
            .fifo()
            .read_exact_async(&mut crc_buf)
            .await
            .map_err(|_| DriverRxError::Discarded)?;

        Ok(total_len)
    }
}
