#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;

use thiserror::Error;

device_driver::create_device!(device_name: FusbLowLevel, manifest: "device.yaml");
pub const FUSB302B_I2C_ADDRESS: u8 = 0x22;

#[derive(Debug, Error)]
pub enum FusbError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Invalid voltage: {0}mV for setting")]
    InvalidVoltage(u16),
    #[error("Invalid current: {0}mA for setting")]
    InvalidCurrent(u16),
    #[error("Feature or specific mode not supported/implemented: {0}")]
    NotSupported(&'static str),
}

#[cfg(feature = "defmt")]
impl<I2cErr> defmt::Format for FusbError<I2cErr> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            FusbError::I2c(_) => defmt::write!(f, "E:I2C"),
            FusbError::InvalidVoltage(v) => defmt::write!(f, "E:V_set({}mV)", v),
            FusbError::InvalidCurrent(c) => defmt::write!(f, "E:I_set({}mA)", c),
            FusbError::NotSupported(s) => defmt::write!(f, "E:NoSupp({=str})", s),
        }
    }
}


pub struct FusbInterface<I2CBus> {
    i2c_bus: I2CBus,
}

impl<I2CBus> FusbInterface<I2CBus> {
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self { i2c_bus }
    }
}

#[path = "."]
mod asynchronous {
    use bisync::asynchronous::*;
    use device_driver::AsyncRegisterInterface as RegisterInterface;
    use embedded_hal_async::i2c::I2c;
    mod driver;
    pub use driver::*;
}
pub use asynchronous::Fusb302b as Fusb302bAsync;

#[path = "."]
mod blocking {
    use bisync::synchronous::*;
    use device_driver::RegisterInterface;
    use embedded_hal::i2c::I2c;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    pub use driver::*;
}
pub use blocking::Fusb302b;