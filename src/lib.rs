#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;

use device_driver::AsyncRegisterInterface;
use embedded_hal_async::i2c::I2c;
use thiserror::Error;
use crate::field_sets::DeviceId;
use usbpd_traits::Driver as PdDriver;

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

