#![cfg_attr(not(test), no_std)]
// #![warn(missing_docs)] // Good to enable later

//! Low-level driver for the FUSB302B USB Type-C controller with PD.
//! This crate provides register and FIFO access via an asynchronous I2C interface.

use embedded_hal_async::i2c::{I2c, Operation};

// This line will generate the `FusbLowLevel` struct and associated field_sets, enums, etc.
// based on the definitions in "device.yaml".
device_driver::create_device!(
    device_name: FusbLowLevel,
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
        Self { i2c, device_address }
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

impl<I2cError> From<I2cError> for DeviceError<I2cError> {
    fn from(value: I2cError) -> Self {
        DeviceError::I2c(value)
    }
}

impl<I2C> device_driver::AsyncRegisterInterface for DeviceInterface<I2C>
where
    I2C: I2c,
{
    type Error = DeviceError<I2C::Error>;
    type AddressType = u8; // Defined in device.yaml config

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32, // All registers are 8 bits as per device.yaml
        data: &[u8],
    ) -> Result<(), Self::Error> {
        if data.len() != 1 {
            // All registers in FUSB302B are 1 byte
            return Err(DeviceError::SliceLength);
        }
        // For FUSB302B, I2C write is: [DEVICE_ADDR_W, REGISTER_ADDR, DATA_BYTE]
        let write_buffer = [address, data[0]];
        self.i2c
            .write(self.device_address, &write_buffer)
            .await
            .map_err(DeviceError::I2c)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32, // All registers are 8 bits
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        if data.len() != 1 {
            // All registers in FUSB302B are 1 byte
            return Err(DeviceError::SliceLength);
        }
        // For FUSB302B, I2C read is:
        // 1. Write: [DEVICE_ADDR_W, REGISTER_ADDR]
        // 2. Read:  [DEVICE_ADDR_R, read data into `data`]
        self.i2c
            .write_read(self.device_address, &[address], data)
            .await
            .map_err(DeviceError::I2c)
    }
}

// The FUSB302B device.yaml does not define any 'command' types,
// so AsyncCommandInterface is not strictly needed.
// If it were, it would look like this:
// impl<I2C> device_driver::AsyncCommandInterface for DeviceInterface<I2C>
// where
//     I2C: i2c::I2c,
// {
//     type Error = DeviceError<I2C::Error>;
//     type AddressType = u8;
//
//     async fn dispatch_command(
//         &mut self,
//         address: Self::AddressType,
//         _size_bits_in: u32,
//         input: &[u8],
//         _size_bits_out: u32,
//         output: &mut [u8],
//     ) -> Result<(), Self::Error> {
//         // Implementation depends on how commands are structured for the device
//         // For FUSB302B, if commands were simple register writes:
//         // let write_buffer = [address, input[0]]; // Assuming 1-byte input
//         // self.i2c.write(self.device_address, &write_buffer).await.map_err(DeviceError::I2c)
//         todo!("Command dispatch not implemented as FUSB302B has no commands in YAML")
//     }
// }

// This trait defines the error type for buffer operations.
impl<I2C> device_driver::BufferInterfaceError for DeviceInterface<I2C>
where
    I2C: I2c, // Add necessary bounds for I2C if any were needed for the error type itself
{
    type Error = DeviceError<I2C::Error>;
}

// AsyncBufferInterface requires BufferInterfaceError to be implemented.
impl<I2C> device_driver::AsyncBufferInterface for DeviceInterface<I2C>
where
    I2C: I2c,
{
    type AddressType = u8; // Defined in device.yaml config for buffers

    async fn write(
        &mut self,
        address: Self::AddressType, // This will be the FIFO address (0x43)
        buf: &[u8],
    ) -> Result<usize, <Self as device_driver::BufferInterfaceError>::Error> { // Use the Error from BufferInterfaceError
        if buf.is_empty() {
            return Ok(0);
        }
        // I2C write to FIFO: [DEVICE_ADDR_W, FIFO_REGISTER_ADDR, DATA_0, ..., DATA_N-1]
        // Using transaction to avoid intermediate buffer for [address] + buf concatenation
        
        // Create a longer-lived binding for the address byte
        let address_byte_slice = [address]; 

        let mut ops = [
            Operation::Write(&address_byte_slice), // Use the longer-lived slice
            Operation::Write(buf),        // Then, send the data to write into the FIFO
        ];
        self.i2c
            .transaction(self.device_address, &mut ops)
            .await
            .map_err(DeviceError::I2c)?;
        Ok(buf.len()) // Assume all bytes are written if transaction succeeds
    }

    async fn read(
        &mut self,
        address: Self::AddressType, // This will be the FIFO address (0x43)
        buf: &mut [u8],
    ) -> Result<usize, <Self as device_driver::BufferInterfaceError>::Error> { // Use the Error from BufferInterfaceError
        if buf.is_empty() {
            return Ok(0);
        }
        // I2C read from FIFO:
        // 1. Write: [DEVICE_ADDR_W, FIFO_REGISTER_ADDR]
        // 2. Read:  [DEVICE_ADDR_R, read data into `buf`]
        self.i2c
            .write_read(self.device_address, &[address], buf)
            .await
            .map_err(DeviceError::I2c)?;
        Ok(buf.len()) // Assume `buf.len()` bytes are read if successful
    }

    async fn flush(&mut self, _address: Self::AddressType) -> Result<(), <Self as device_driver::BufferInterfaceError>::Error> { // Use the Error from BufferInterfaceError
        // The FUSB302B has TX_FLUSH and RX_FLUSH bits in Control0 and Control1 registers.
        // This `flush` is typically for the bus itself.
        // If specific FIFO flush is needed, it should be done via register writes.
        Ok(())
    }
}
