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

    // A mask to isolate the bits that identify a Start-of-Packet family token.
    pub const SOP_MASK: u8 = 0b1110_0000;

    // The expected pattern after masking for any SOP*, SOP', or SOP'' token.
    pub const SOP_PATTERN: u8 = 0b1110_0000;
}

use device_driver::{AsyncBufferInterface, AsyncRegisterInterface, BufferInterfaceError};
use embedded_hal_async::i2c::I2c;
use thiserror::Error;

use crate::field_sets::{DeviceId, Mask, Maska, Maskb};
use embassy_time::{Duration, Instant, Timer};
use usbpd_traits::{Driver as SinkDriver, DriverRxError, DriverTxError};

device_driver::create_device!(device_name: FusbLowLevel, manifest: "device.yaml");
pub const FUSB302B_I2C_ADDRESS: u8 = 0x22;

/// Convert BcLvl enum to comparable u8 value
fn bc_lvl_to_u8(lvl: BcLvl) -> u8 {
    match lvl {
        BcLvl::LessThan200MV => 0,
        BcLvl::Between200And660MV => 1,
        BcLvl::Between660And1230MV => 2,
        BcLvl::GreaterThan1230MV => 3,
    }
}

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

/// Detected CC pin orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcPin {
    Cc1,
    Cc2,
}

impl<I2CBus, E> Fusb302b<I2CBus, E>
where
    I2CBus: I2c<Error = E> + 'static,
    E: core::fmt::Debug,
{
    pub async fn init(i2c: I2CBus) -> Result<Self, FusbError<E>> {
        let mut driver = Self {
            ll: FusbLowLevel::new(DeviceInterface::new(i2c)),
            _marker: core::marker::PhantomData,
        };

        // Fully reset the FUSB302B
        driver
            .ll
            .reset()
            .write_async(|r| r.set_sw_res(true))
            .await?;

        Timer::after_millis(10).await;

        // Verify device is responding
        let device_id = driver.ll.device_id().read_async().await?;
        // Check that we get a valid known version (A, B, or C)
        match device_id.version_id() {
            Fusb302Version::VersionA | Fusb302Version::VersionB | Fusb302Version::VersionC => {}
            Fusb302Version::UnknownOrFutureVersion(_) => {
                return Err(FusbError::LenExceedsBuffer); // TODO: add proper error variant
            }
        }

        // Turn on all power
        driver
            .ll
            .power()
            .write_async(|r| {
                r.set_pwr_0_bandgap_and_wake_enable(true);
                r.set_pwr_1_receiver_and_measure_refs_enable(true);
                r.set_pwr_2_measure_block_power_enable(true);
                r.set_pwr_3_internal_oscillator_enable(true);
            })
            .await?;

        // Set interrupt masks to 0 (all interrupts enabled)
        driver.ll.mask().write_async(|r| *r = Mask::new()).await?;
        driver.ll.maska().write_async(|r| *r = Maska::new()).await?;
        driver.ll.maskb().write_async(|r| *r = Maskb::new()).await?;

        // Unmask interrupts
        driver
            .ll
            .control_0()
            .write_async(|r| {
                r.set_int_mask(false);
            })
            .await?;

        // Configure hardware auto-retry: 2 retries per USB PD spec (nRetryCount = 2)
        driver
            .ll
            .control_3()
            .write_async(|r| {
                r.set_auto_retry(true);
                r.set_n_retries(RetryCount::TwoRetries);
                r.set_auto_softreset(false); // Let policy engine handle
                r.set_auto_hardreset(false); // Let policy engine handle
            })
            .await?;

        // Flush the RX buffer
        driver
            .ll
            .control_1()
            .write_async(|r| r.set_rx_flush(true))
            .await?;

        // Detect and select CC line
        driver.detect_cc_pin().await?;

        // Reset PD logic
        driver
            .ll
            .reset()
            .write_async(|r| r.set_pd_reset(true))
            .await?;

        Ok(driver)
    }

    /// Detect which CC pin is connected and configure switches accordingly
    async fn detect_cc_pin(&mut self) -> Result<CcPin, FusbError<E>> {
        // Measure CC1: PDWN1|PDWN2|MEAS_CC1
        self.ll
            .switches_0()
            .write_async(|r| {
                r.set_pdwn_1(true);
                r.set_pdwn_2(true);
                r.set_meas_cc_1(true);
                r.set_meas_cc_2(false);
            })
            .await?;

        Timer::after_millis(10).await;

        let cc1 = bc_lvl_to_u8(self.ll.status_0().read_async().await?.bc_lvl());

        // Measure CC2: PDWN1|PDWN2|MEAS_CC2
        self.ll
            .switches_0()
            .write_async(|r| {
                r.set_pdwn_1(true);
                r.set_pdwn_2(true);
                r.set_meas_cc_1(false);
                r.set_meas_cc_2(true);
            })
            .await?;

        Timer::after_millis(10).await;

        let cc2 = bc_lvl_to_u8(self.ll.status_0().read_async().await?.bc_lvl());

        // Select the CC line with higher voltage (indicates connection)
        let selected_cc = if cc1 > cc2 { CcPin::Cc1 } else { CcPin::Cc2 };

        // Configure switches for the selected CC line
        match selected_cc {
            CcPin::Cc1 => {
                // TX on CC1, AUTO_CRC enabled
                self.ll
                    .switches_1()
                    .write_async(|r| {
                        r.set_txcc_1(true);
                        r.set_txcc_2(false);
                        r.set_auto_crc(true);
                    })
                    .await?;
                // Measure CC1
                self.ll
                    .switches_0()
                    .write_async(|r| {
                        r.set_pdwn_1(true);
                        r.set_pdwn_2(true);
                        r.set_meas_cc_1(true);
                        r.set_meas_cc_2(false);
                    })
                    .await?;
            }
            CcPin::Cc2 => {
                // TX on CC2, AUTO_CRC enabled
                self.ll
                    .switches_1()
                    .write_async(|r| {
                        r.set_txcc_1(false);
                        r.set_txcc_2(true);
                        r.set_auto_crc(true);
                    })
                    .await?;
                // Measure CC2
                self.ll
                    .switches_0()
                    .write_async(|r| {
                        r.set_pdwn_1(true);
                        r.set_pdwn_2(true);
                        r.set_meas_cc_1(false);
                        r.set_meas_cc_2(true);
                    })
                    .await?;
            }
        }

        Ok(selected_cc)
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
    const HAS_AUTO_RETRY: bool = true;

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
            .modify_async(|r| r.set_pwr_3_internal_oscillator_enable(true))
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
                .modify_async(|r| r.set_pwr_3_internal_oscillator_enable(false))
                .await
                .ok();
            return Err(DriverTxError::Discarded);
        }

        // With hardware auto-retry (up to 3 attempts), we need more time than the original 5ms
        let deadline = Instant::now() + Duration::from_millis(15);
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
            if irqa.i_retryfail() {
                // Hardware auto-retry exhausted all attempts without receiving GoodCRC
                self.ll
                    .interrupta()
                    .modify_async(|r| r.set_i_retryfail(true))
                    .await
                    .ok();
                tx_result = Err(DriverTxError::Discarded);
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
            .modify_async(|r| r.set_pwr_3_internal_oscillator_enable(false))
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

        // Check if the received byte is a valid Start-of-Packet token.
        if (token_buf[0] & token::SOP_MASK) != token::SOP_PATTERN {
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
