#![cfg_attr(not(test), no_std)]
// #![warn(missing_docs)] // Good to enable later

//! Low-level driver for the FUSB302B USB Type-C controller with PD.
//! This crate provides register and FIFO access via an asynchronous I2C interface.

use embedded_hal_async::i2c::{I2c, Operation};

// This line will generate the `FusbLowLevel` struct and associated field_sets, enums, etc.
// based on the definitions in "device.yaml".
device_driver::create_device!(
    device_name: Device,
    manifest: "device.yaml"
);

/// The I2C device address for the FUSB302B (common variants like BUCX, BMPX, BVMPX).
/// Other variants might have addresses 0x23, 0x24, or 0x25.
/// Refer to Table 15 in the FUSB302B datasheet.
pub const DEFAULT_DEVICE_I2C_ADDR: u8 = 0x22;

/// The asynchronous I2C wrapper interface to the FUSB302B driver.
#[derive(Debug)]
pub struct DeviceInterface<I2C> {
    i2c: I2C,
    device_address: u8,
}

impl<I2C> DeviceInterface<I2C>
where
    I2C: I2c,
{
    /// Construct a new instance of the device interface.
    ///
    /// # Arguments
    ///
    /// * `i2c`: The asynchronous I2C bus peripheral.
    /// * `device_address`: The 7-bit I2C address of the FUSB302B device.
    ///   Use `DEFAULT_DEVICE_I2C_ADDR` (0x22) if unsure, or check your FUSB302B variant.
    pub fn new(i2c: I2C, device_address: u8) -> Self {
        Self {
            i2c,
            device_address,
        }
    }

    /// Consumes the interface and returns the underlying I2C bus.
    pub fn release(self) -> I2C {
        self.i2c
    }
}

/// Low-level interface error that wraps the I2C bus error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceError<I2cError> {
    /// I2C communication error.
    I2c(I2cError),
    /// Data slice provided was too short or an unexpected length.
    SliceLength,
}
