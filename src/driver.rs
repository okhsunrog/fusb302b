use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::{FUSB302B_I2C_ADDRESS, FusbError, FusbInterface, FusbLowLevel};

#[bisync]
impl<I2CBus, E> RegisterInterface for FusbInterface<I2CBus>
where
    I2CBus: I2c<Error = E>,
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
        self.i2c_bus
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
            return Err(FusbError::NotSupported("Write data length exceeds buffer"));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(FUSB302B_I2C_ADDRESS, &buffer[..1 + data.len()])
            .await
            .map_err(FusbError::I2c)
    }
}

pub struct Fusb302b<
    I2CImpl: RegisterInterface<AddressType = u8, Error = FusbError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as RegisterInterface>::Error,
> {
    pub ll: FusbLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

impl<I2CBus, E> Fusb302b<FusbInterface<I2CBus>, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: FusbLowLevel::new(FusbInterface::new(i2c)),
            _marker: core::marker::PhantomData,
        }
    }
}

pub trait CurrentAxpDriverInterface<E>:
    RegisterInterface<AddressType = u8, Error = FusbError<E>>
{
}

impl<T, E> CurrentAxpDriverInterface<E> for T
where
    T: RegisterInterface<AddressType = u8, Error = FusbError<E>>,
    E: core::fmt::Debug,
{
}

include!("bisync_helpers.rs");

// impl<I2CImpl, I2CBusErr> Axp192<I2CImpl, I2CBusErr>
// where
//     I2CImpl: CurrentAxpDriverInterface<I2CBusErr>,
//     I2CBusErr: core::fmt::Debug,
// {


//     #[bisync]
//     pub async fn set_dcdc_enable(
//         &mut self,
//         dc: DcId,
//         enable: bool,
//     ) -> Result<(), AxpError<I2CBusErr>> {
//         let mut op = self.ll.power_output_control();
//         modify_internal(&mut op, |r| match dc {
//             DcId::Dcdc1 => r.set_dcdc_1_output_enable(enable),
//             DcId::Dcdc2 => r.set_dcdc_2_output_enable(enable),
//             DcId::Dcdc3 => r.set_dcdc_3_output_enable(enable),
//         })
//         .await
//     }

// }