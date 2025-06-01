///Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.device_id().read()?;
        callback(1 + 0 * 0, "device_id", reg.into());
        let reg = self.switches_0().read()?;
        callback(2 + 0 * 0, "switches_0", reg.into());
        let reg = self.switches_1().read()?;
        callback(3 + 0 * 0, "switches_1", reg.into());
        let reg = self.measure().read()?;
        callback(4 + 0 * 0, "measure", reg.into());
        let reg = self.slice().read()?;
        callback(5 + 0 * 0, "slice", reg.into());
        let reg = self.control_0().read()?;
        callback(6 + 0 * 0, "control_0", reg.into());
        let reg = self.control_1().read()?;
        callback(7 + 0 * 0, "control_1", reg.into());
        let reg = self.control_2().read()?;
        callback(8 + 0 * 0, "control_2", reg.into());
        let reg = self.control_3().read()?;
        callback(9 + 0 * 0, "control_3", reg.into());
        let reg = self.mask().read()?;
        callback(10 + 0 * 0, "mask", reg.into());
        let reg = self.power().read()?;
        callback(11 + 0 * 0, "power", reg.into());
        let reg = self.reset().read()?;
        callback(12 + 0 * 0, "reset", reg.into());
        let reg = self.ocpreg().read()?;
        callback(13 + 0 * 0, "ocpreg", reg.into());
        let reg = self.maska().read()?;
        callback(14 + 0 * 0, "maska", reg.into());
        let reg = self.maskb().read()?;
        callback(15 + 0 * 0, "maskb", reg.into());
        let reg = self.control_4().read()?;
        callback(16 + 0 * 0, "control_4", reg.into());
        let reg = self.status_0_a().read()?;
        callback(60 + 0 * 0, "status_0_a", reg.into());
        let reg = self.status_1_a().read()?;
        callback(61 + 0 * 0, "status_1_a", reg.into());
        let reg = self.interrupta().read()?;
        callback(62 + 0 * 0, "interrupta", reg.into());
        let reg = self.interruptb().read()?;
        callback(63 + 0 * 0, "interruptb", reg.into());
        let reg = self.status_0().read()?;
        callback(64 + 0 * 0, "status_0", reg.into());
        let reg = self.status_1().read()?;
        callback(65 + 0 * 0, "status_1", reg.into());
        let reg = self.interrupt().read()?;
        callback(66 + 0 * 0, "interrupt", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.device_id().read_async().await?;
        callback(1 + 0 * 0, "device_id", reg.into());
        let reg = self.switches_0().read_async().await?;
        callback(2 + 0 * 0, "switches_0", reg.into());
        let reg = self.switches_1().read_async().await?;
        callback(3 + 0 * 0, "switches_1", reg.into());
        let reg = self.measure().read_async().await?;
        callback(4 + 0 * 0, "measure", reg.into());
        let reg = self.slice().read_async().await?;
        callback(5 + 0 * 0, "slice", reg.into());
        let reg = self.control_0().read_async().await?;
        callback(6 + 0 * 0, "control_0", reg.into());
        let reg = self.control_1().read_async().await?;
        callback(7 + 0 * 0, "control_1", reg.into());
        let reg = self.control_2().read_async().await?;
        callback(8 + 0 * 0, "control_2", reg.into());
        let reg = self.control_3().read_async().await?;
        callback(9 + 0 * 0, "control_3", reg.into());
        let reg = self.mask().read_async().await?;
        callback(10 + 0 * 0, "mask", reg.into());
        let reg = self.power().read_async().await?;
        callback(11 + 0 * 0, "power", reg.into());
        let reg = self.reset().read_async().await?;
        callback(12 + 0 * 0, "reset", reg.into());
        let reg = self.ocpreg().read_async().await?;
        callback(13 + 0 * 0, "ocpreg", reg.into());
        let reg = self.maska().read_async().await?;
        callback(14 + 0 * 0, "maska", reg.into());
        let reg = self.maskb().read_async().await?;
        callback(15 + 0 * 0, "maskb", reg.into());
        let reg = self.control_4().read_async().await?;
        callback(16 + 0 * 0, "control_4", reg.into());
        let reg = self.status_0_a().read_async().await?;
        callback(60 + 0 * 0, "status_0_a", reg.into());
        let reg = self.status_1_a().read_async().await?;
        callback(61 + 0 * 0, "status_1_a", reg.into());
        let reg = self.interrupta().read_async().await?;
        callback(62 + 0 * 0, "interrupta", reg.into());
        let reg = self.interruptb().read_async().await?;
        callback(63 + 0 * 0, "interruptb", reg.into());
        let reg = self.status_0().read_async().await?;
        callback(64 + 0 * 0, "status_0", reg.into());
        let reg = self.status_1().read_async().await?;
        callback(65 + 0 * 0, "status_1", reg.into());
        let reg = self.interrupt().read_async().await?;
        callback(66 + 0 * 0, "interrupt", reg.into());
        Ok(())
    }
    ///Device Identification Register
    pub fn device_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DeviceId,
        ::device_driver::RO,
    > {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DeviceId,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::DeviceId::new)
    }
    ///Switch Control Register 0
    pub fn switches_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Switches0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Switches0,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Switches0::new)
    }
    ///Switch Control Register 1
    pub fn switches_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Switches1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Switches1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Switches1::new)
    }
    ///Measure Control Register
    pub fn measure(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Measure,
        ::device_driver::RW,
    > {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Measure,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Measure::new)
    }
    ///Slice Control Register
    pub fn slice(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Slice,
        ::device_driver::RW,
    > {
        let address = self.base_address + 5;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Slice,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Slice::new)
    }
    ///Control Register 0
    pub fn control_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Control0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Control0,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Control0::new)
    }
    ///Control Register 1
    pub fn control_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Control1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Control1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Control1::new)
    }
    ///Control Register 2
    pub fn control_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Control2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Control2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Control2::new)
    }
    ///Control Register 3
    pub fn control_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Control3,
        ::device_driver::RW,
    > {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Control3,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Control3::new)
    }
    ///Interrupt Mask Register
    pub fn mask(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Mask,
        ::device_driver::RW,
    > {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Mask,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Mask::new)
    }
    ///Power Control Register
    pub fn power(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Power,
        ::device_driver::RW,
    > {
        let address = self.base_address + 11;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Power,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Power::new)
    }
    ///Reset Control Register
    pub fn reset(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Reset,
        ::device_driver::WO,
    > {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Reset,
            ::device_driver::WO,
        >::new(self.interface(), address as u8, field_sets::Reset::new)
    }
    ///Over-Current Protection Register
    pub fn ocpreg(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Ocpreg,
        ::device_driver::RW,
    > {
        let address = self.base_address + 13;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Ocpreg,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Ocpreg::new)
    }
    ///Interrupt Mask Register A
    pub fn maska(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Maska,
        ::device_driver::RW,
    > {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Maska,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Maska::new)
    }
    ///Interrupt Mask Register B
    pub fn maskb(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Maskb,
        ::device_driver::RW,
    > {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Maskb,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Maskb::new)
    }
    ///Control Register 4
    pub fn control_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Control4,
        ::device_driver::RW,
    > {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Control4,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Control4::new)
    }
    ///Status Register 0A
    pub fn status_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Status0A,
        ::device_driver::RO,
    > {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Status0A,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Status0A::new)
    }
    ///Status Register 1A
    pub fn status_1_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Status1A,
        ::device_driver::RO,
    > {
        let address = self.base_address + 61;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Status1A,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Status1A::new)
    }
    ///Interrupt Register A (read/clear)
    pub fn interrupta(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Interrupta,
        ::device_driver::RW,
    > {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Interrupta,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Interrupta::new)
    }
    ///Interrupt Register B (read/clear)
    pub fn interruptb(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Interruptb,
        ::device_driver::RW,
    > {
        let address = self.base_address + 63;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Interruptb,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Interruptb::new)
    }
    ///Status Register 0
    pub fn status_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Status0,
        ::device_driver::RO,
    > {
        let address = self.base_address + 64;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Status0,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Status0::new)
    }
    ///Status Register 1
    pub fn status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Status1,
        ::device_driver::RO,
    > {
        let address = self.base_address + 65;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Status1,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Status1::new)
    }
    ///Interrupt Status Register (read/clear)
    pub fn interrupt(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Interrupt,
        ::device_driver::RW,
    > {
        let address = self.base_address + 66;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Interrupt,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Interrupt::new)
    }
    ///FIFO buffer for USB PD communication (write to Tx, read from Rx).
    pub fn fifo(
        &mut self,
    ) -> ::device_driver::BufferOperation<'_, I, u8, ::device_driver::RW> {
        let address = self.base_address + 67;
        ::device_driver::BufferOperation::<
            '_,
            I,
            u8,
            ::device_driver::RW,
        >::new(self.interface(), address as u8)
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    use super::*;
    ///Device Identification Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DeviceId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DeviceId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DeviceId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `device_id` field of the register.
        ///
        ///Device ID value (e.g., 0x9x for FUSB302B variants).
        pub fn device_id(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `device_id` field of the register.
        ///
        ///Device ID value (e.g., 0x9x for FUSB302B variants).
        pub fn set_device_id(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DeviceId {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DeviceId> for [u8; 1] {
        fn from(val: DeviceId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DeviceId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DeviceId");
            {
                d.field("device_id", &self.device_id());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DeviceId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DeviceId { ");
            defmt::write!(f, "device_id: {=u8}, ", &self.device_id());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DeviceId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DeviceId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DeviceId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DeviceId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DeviceId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DeviceId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DeviceId {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Switch Control Register 0
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Switches0 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Switches0 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Switches0 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pu_en_2` field of the register.
        ///
        ///Pull-up enable for CC2 (true: apply host pull-up current).
        pub fn pu_en_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `pu_en_1` field of the register.
        ///
        ///Pull-up enable for CC1 (true: apply host pull-up current).
        pub fn pu_en_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `vconn_cc_2` field of the register.
        ///
        ///VCONN enable for CC2 (true: turn on VCONN current).
        pub fn vconn_cc_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `vconn_cc_1` field of the register.
        ///
        ///VCONN enable for CC1 (true: turn on VCONN current).
        pub fn vconn_cc_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `meas_cc_2` field of the register.
        ///
        ///Measure CC2 (true: monitor/measure voltage on CC2).
        pub fn meas_cc_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `meas_cc_1` field of the register.
        ///
        ///Measure CC1 (true: monitor/measure voltage on CC1).
        pub fn meas_cc_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `pdwn_2` field of the register.
        ///
        ///Pull-down for CC2 (true: device pull-down on).
        pub fn pdwn_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `pdwn_1` field of the register.
        ///
        ///Pull-down for CC1 (true: device pull-down on).
        pub fn pdwn_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `pu_en_2` field of the register.
        ///
        ///Pull-up enable for CC2 (true: apply host pull-up current).
        pub fn set_pu_en_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `pu_en_1` field of the register.
        ///
        ///Pull-up enable for CC1 (true: apply host pull-up current).
        pub fn set_pu_en_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `vconn_cc_2` field of the register.
        ///
        ///VCONN enable for CC2 (true: turn on VCONN current).
        pub fn set_vconn_cc_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `vconn_cc_1` field of the register.
        ///
        ///VCONN enable for CC1 (true: turn on VCONN current).
        pub fn set_vconn_cc_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `meas_cc_2` field of the register.
        ///
        ///Measure CC2 (true: monitor/measure voltage on CC2).
        pub fn set_meas_cc_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `meas_cc_1` field of the register.
        ///
        ///Measure CC1 (true: monitor/measure voltage on CC1).
        pub fn set_meas_cc_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `pdwn_2` field of the register.
        ///
        ///Pull-down for CC2 (true: device pull-down on).
        pub fn set_pdwn_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `pdwn_1` field of the register.
        ///
        ///Pull-down for CC1 (true: device pull-down on).
        pub fn set_pdwn_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Switches0 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Switches0> for [u8; 1] {
        fn from(val: Switches0) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Switches0 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Switches0");
            {
                d.field("pu_en_2", &self.pu_en_2());
            }
            {
                d.field("pu_en_1", &self.pu_en_1());
            }
            {
                d.field("vconn_cc_2", &self.vconn_cc_2());
            }
            {
                d.field("vconn_cc_1", &self.vconn_cc_1());
            }
            {
                d.field("meas_cc_2", &self.meas_cc_2());
            }
            {
                d.field("meas_cc_1", &self.meas_cc_1());
            }
            {
                d.field("pdwn_2", &self.pdwn_2());
            }
            {
                d.field("pdwn_1", &self.pdwn_1());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Switches0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Switches0 { ");
            defmt::write!(f, "pu_en_2: {=bool}, ", &self.pu_en_2());
            defmt::write!(f, "pu_en_1: {=bool}, ", &self.pu_en_1());
            defmt::write!(f, "vconn_cc_2: {=bool}, ", &self.vconn_cc_2());
            defmt::write!(f, "vconn_cc_1: {=bool}, ", &self.vconn_cc_1());
            defmt::write!(f, "meas_cc_2: {=bool}, ", &self.meas_cc_2());
            defmt::write!(f, "meas_cc_1: {=bool}, ", &self.meas_cc_1());
            defmt::write!(f, "pdwn_2: {=bool}, ", &self.pdwn_2());
            defmt::write!(f, "pdwn_1: {=bool}, ", &self.pdwn_1());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Switches0 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Switches0 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Switches0 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Switches0 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Switches0 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Switches0 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Switches0 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Switch Control Register 1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Switches1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Switches1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Switches1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `powerrole` field of the register.
        ///
        ///Power role for GoodCRC packet (true: Source, false: Sink for SOP).
        pub fn powerrole(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `specrev` field of the register.
        ///
        ///Specification revision for GoodCRC packet.
        pub fn specrev(
            &self,
        ) -> Result<super::SpecRev, <super::SpecRev as TryFrom<u8>>::Error> {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 7)
            };
            raw.try_into()
        }
        ///Read the `datarole` field of the register.
        ///
        ///Data role for GoodCRC packet (true: SRC, false: SNK for SOP).
        pub fn datarole(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `auto_crc` field of the register.
        ///
        ///Automatic CRC handling (true: auto-send GoodCRC on valid packet).
        pub fn auto_crc(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `txcc_2` field of the register.
        ///
        ///Transmit on CC2 (true: enable BMC transmit driver).
        pub fn txcc_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `txcc_1` field of the register.
        ///
        ///Transmit on CC1 (true: enable BMC transmit driver).
        pub fn txcc_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `powerrole` field of the register.
        ///
        ///Power role for GoodCRC packet (true: Source, false: Sink for SOP).
        pub fn set_powerrole(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `specrev` field of the register.
        ///
        ///Specification revision for GoodCRC packet.
        pub fn set_specrev(&mut self, value: super::SpecRev) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 7, &mut self.bits)
            };
        }
        ///Write the `datarole` field of the register.
        ///
        ///Data role for GoodCRC packet (true: SRC, false: SNK for SOP).
        pub fn set_datarole(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `auto_crc` field of the register.
        ///
        ///Automatic CRC handling (true: auto-send GoodCRC on valid packet).
        pub fn set_auto_crc(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `txcc_2` field of the register.
        ///
        ///Transmit on CC2 (true: enable BMC transmit driver).
        pub fn set_txcc_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `txcc_1` field of the register.
        ///
        ///Transmit on CC1 (true: enable BMC transmit driver).
        pub fn set_txcc_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Switches1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Switches1> for [u8; 1] {
        fn from(val: Switches1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Switches1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Switches1");
            {
                d.field("powerrole", &self.powerrole());
            }
            {
                d.field("specrev", &self.specrev());
            }
            {
                d.field("datarole", &self.datarole());
            }
            {
                d.field("auto_crc", &self.auto_crc());
            }
            {
                d.field("txcc_2", &self.txcc_2());
            }
            {
                d.field("txcc_1", &self.txcc_1());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Switches1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Switches1 { ");
            defmt::write!(f, "powerrole: {=bool}, ", &self.powerrole());
            defmt::write!(f, "specrev: {}, ", &self.specrev());
            defmt::write!(f, "datarole: {=bool}, ", &self.datarole());
            defmt::write!(f, "auto_crc: {=bool}, ", &self.auto_crc());
            defmt::write!(f, "txcc_2: {=bool}, ", &self.txcc_2());
            defmt::write!(f, "txcc_1: {=bool}, ", &self.txcc_1());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Switches1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Switches1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Switches1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Switches1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Switches1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Switches1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Switches1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Measure Control Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Measure {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Measure {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Measure {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `meas_vbus` field of the register.
        ///
        ///Measure VBUS (true: measure VBUS voltage).
        pub fn meas_vbus(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `mdac` field of the register.
        ///
        ///MDAC setting for voltage measurement threshold.
        pub fn mdac(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 6)
            };
            raw
        }
        ///Write the `meas_vbus` field of the register.
        ///
        ///Measure VBUS (true: measure VBUS voltage).
        pub fn set_meas_vbus(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `mdac` field of the register.
        ///
        ///MDAC setting for voltage measurement threshold.
        pub fn set_mdac(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 6, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Measure {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Measure> for [u8; 1] {
        fn from(val: Measure) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Measure {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Measure");
            {
                d.field("meas_vbus", &self.meas_vbus());
            }
            {
                d.field("mdac", &self.mdac());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Measure {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Measure { ");
            defmt::write!(f, "meas_vbus: {=bool}, ", &self.meas_vbus());
            defmt::write!(f, "mdac: {=u8}, ", &self.mdac());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Measure {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Measure {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Measure {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Measure {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Measure {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Measure {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Measure {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Slice Control Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slice {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Slice {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Slice {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `sdac_hys` field of the register.
        ///
        ///SDAC hysteresis setting.
        pub fn sdac_hys(&self) -> super::SdacHys {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `sdac` field of the register.
        ///
        ///SDAC setting for BMC slicer DAC threshold.
        pub fn sdac(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 6)
            };
            raw
        }
        ///Write the `sdac_hys` field of the register.
        ///
        ///SDAC hysteresis setting.
        pub fn set_sdac_hys(&mut self, value: super::SdacHys) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 8, &mut self.bits)
            };
        }
        ///Write the `sdac` field of the register.
        ///
        ///SDAC setting for BMC slicer DAC threshold.
        pub fn set_sdac(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 6, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Slice {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Slice> for [u8; 1] {
        fn from(val: Slice) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Slice {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Slice");
            {
                d.field("sdac_hys", &self.sdac_hys());
            }
            {
                d.field("sdac", &self.sdac());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Slice {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Slice { ");
            defmt::write!(f, "sdac_hys: {}, ", &self.sdac_hys());
            defmt::write!(f, "sdac: {=u8}, ", &self.sdac());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Slice {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Slice {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Slice {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Slice {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Slice {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Slice {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Slice {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Control Register 0
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control0 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Control0 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Control0 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `tx_flush` field of the register.
        ///
        ///Flush transmit FIFO (true: self-clearing flush).
        pub fn tx_flush(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `int_mask` field of the register.
        ///
        ///Interrupt mask (true: mask all interrupts).
        pub fn int_mask(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `host_cur` field of the register.
        ///
        ///Host current setting for pull-ups.
        pub fn host_cur(&self) -> super::HostCurrent {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `auto_pre` field of the register.
        ///
        ///Auto preamble (true: auto-start transmitter on good CRC).
        pub fn auto_pre(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `tx_start` field of the register.
        ///
        ///Start transmission (true: initiate packet transmission).
        pub fn tx_start(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `tx_flush` field of the register.
        ///
        ///Flush transmit FIFO (true: self-clearing flush).
        pub fn set_tx_flush(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `int_mask` field of the register.
        ///
        ///Interrupt mask (true: mask all interrupts).
        pub fn set_int_mask(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `host_cur` field of the register.
        ///
        ///Host current setting for pull-ups.
        pub fn set_host_cur(&mut self, value: super::HostCurrent) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `auto_pre` field of the register.
        ///
        ///Auto preamble (true: auto-start transmitter on good CRC).
        pub fn set_auto_pre(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `tx_start` field of the register.
        ///
        ///Start transmission (true: initiate packet transmission).
        pub fn set_tx_start(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Control0 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Control0> for [u8; 1] {
        fn from(val: Control0) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Control0 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Control0");
            {
                d.field("tx_flush", &self.tx_flush());
            }
            {
                d.field("int_mask", &self.int_mask());
            }
            {
                d.field("host_cur", &self.host_cur());
            }
            {
                d.field("auto_pre", &self.auto_pre());
            }
            {
                d.field("tx_start", &self.tx_start());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Control0 { ");
            defmt::write!(f, "tx_flush: {=bool}, ", &self.tx_flush());
            defmt::write!(f, "int_mask: {=bool}, ", &self.int_mask());
            defmt::write!(f, "host_cur: {}, ", &self.host_cur());
            defmt::write!(f, "auto_pre: {=bool}, ", &self.auto_pre());
            defmt::write!(f, "tx_start: {=bool}, ", &self.tx_start());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Control0 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Control0 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Control0 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Control0 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Control0 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Control0 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Control0 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Control Register 1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Control1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Control1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `ensop_2_db` field of the register.
        ///
        ///Enable SOP'' Debug packets (true: enabled).
        pub fn ensop_2_db(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `ensop_1_db` field of the register.
        ///
        ///Enable SOP' Debug packets (true: enabled).
        pub fn ensop_1_db(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `bist_mode_2` field of the register.
        ///
        ///BIST Mode 01s pattern (true: enabled for testing).
        pub fn bist_mode_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `rx_flush` field of the register.
        ///
        ///Flush receive FIFO (true: self-clearing flush).
        pub fn rx_flush(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `ensop_2` field of the register.
        ///
        ///Enable SOP'' packets (true: enabled).
        pub fn ensop_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `ensop_1` field of the register.
        ///
        ///Enable SOP' packets (true: enabled).
        pub fn ensop_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `ensop_2_db` field of the register.
        ///
        ///Enable SOP'' Debug packets (true: enabled).
        pub fn set_ensop_2_db(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `ensop_1_db` field of the register.
        ///
        ///Enable SOP' Debug packets (true: enabled).
        pub fn set_ensop_1_db(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `bist_mode_2` field of the register.
        ///
        ///BIST Mode 01s pattern (true: enabled for testing).
        pub fn set_bist_mode_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `rx_flush` field of the register.
        ///
        ///Flush receive FIFO (true: self-clearing flush).
        pub fn set_rx_flush(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `ensop_2` field of the register.
        ///
        ///Enable SOP'' packets (true: enabled).
        pub fn set_ensop_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `ensop_1` field of the register.
        ///
        ///Enable SOP' packets (true: enabled).
        pub fn set_ensop_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Control1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Control1> for [u8; 1] {
        fn from(val: Control1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Control1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Control1");
            {
                d.field("ensop_2_db", &self.ensop_2_db());
            }
            {
                d.field("ensop_1_db", &self.ensop_1_db());
            }
            {
                d.field("bist_mode_2", &self.bist_mode_2());
            }
            {
                d.field("rx_flush", &self.rx_flush());
            }
            {
                d.field("ensop_2", &self.ensop_2());
            }
            {
                d.field("ensop_1", &self.ensop_1());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Control1 { ");
            defmt::write!(f, "ensop_2_db: {=bool}, ", &self.ensop_2_db());
            defmt::write!(f, "ensop_1_db: {=bool}, ", &self.ensop_1_db());
            defmt::write!(f, "bist_mode_2: {=bool}, ", &self.bist_mode_2());
            defmt::write!(f, "rx_flush: {=bool}, ", &self.rx_flush());
            defmt::write!(f, "ensop_2: {=bool}, ", &self.ensop_2());
            defmt::write!(f, "ensop_1: {=bool}, ", &self.ensop_1());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Control1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Control1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Control1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Control1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Control1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Control1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Control1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Control Register 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Control2 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Control2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `tog_save_pwr` field of the register.
        ///
        ///Toggle save power setting.
        pub fn tog_save_pwr(&self) -> super::TogSavePwr {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `tog_rd_only` field of the register.
        ///
        ///Toggle Rd only (true: stop on Rd only).
        pub fn tog_rd_only(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `wake_en` field of the register.
        ///
        ///Wake detection enable (true: enabled).
        pub fn wake_en(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `mode` field of the register.
        ///
        ///Toggle mode selection.
        pub fn mode(
            &self,
        ) -> Result<super::ToggleMode, <super::ToggleMode as TryFrom<u8>>::Error> {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 3)
            };
            raw.try_into()
        }
        ///Read the `toggle` field of the register.
        ///
        ///Enable toggle (true: autonomous DRP/SNK/SRC toggle).
        pub fn toggle(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `tog_save_pwr` field of the register.
        ///
        ///Toggle save power setting.
        pub fn set_tog_save_pwr(&mut self, value: super::TogSavePwr) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 8, &mut self.bits)
            };
        }
        ///Write the `tog_rd_only` field of the register.
        ///
        ///Toggle Rd only (true: stop on Rd only).
        pub fn set_tog_rd_only(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `wake_en` field of the register.
        ///
        ///Wake detection enable (true: enabled).
        pub fn set_wake_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `mode` field of the register.
        ///
        ///Toggle mode selection.
        pub fn set_mode(&mut self, value: super::ToggleMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 3, &mut self.bits)
            };
        }
        ///Write the `toggle` field of the register.
        ///
        ///Enable toggle (true: autonomous DRP/SNK/SRC toggle).
        pub fn set_toggle(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Control2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Control2> for [u8; 1] {
        fn from(val: Control2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Control2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Control2");
            {
                d.field("tog_save_pwr", &self.tog_save_pwr());
            }
            {
                d.field("tog_rd_only", &self.tog_rd_only());
            }
            {
                d.field("wake_en", &self.wake_en());
            }
            {
                d.field("mode", &self.mode());
            }
            {
                d.field("toggle", &self.toggle());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Control2 { ");
            defmt::write!(f, "tog_save_pwr: {}, ", &self.tog_save_pwr());
            defmt::write!(f, "tog_rd_only: {=bool}, ", &self.tog_rd_only());
            defmt::write!(f, "wake_en: {=bool}, ", &self.wake_en());
            defmt::write!(f, "mode: {}, ", &self.mode());
            defmt::write!(f, "toggle: {=bool}, ", &self.toggle());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Control2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Control2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Control2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Control2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Control2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Control2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Control2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Control Register 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control3 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Control3 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Control3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `send_hard_reset` field of the register.
        ///
        ///Send hard reset (true: initiate, self-clearing).
        pub fn send_hard_reset(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `bist_tmode` field of the register.
        ///
        ///BIST test mode (true: clear RxFIFO after GoodCRC).
        pub fn bist_tmode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `auto_hardreset` field of the register.
        ///
        ///Auto hard reset on soft reset fail (true: enabled).
        pub fn auto_hardreset(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `auto_softreset` field of the register.
        ///
        ///Auto soft reset on retries fail (true: enabled).
        pub fn auto_softreset(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `n_retries` field of the register.
        ///
        ///Number of packet retries.
        pub fn n_retries(&self) -> super::RetryCount {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `auto_retry` field of the register.
        ///
        ///Auto retry on no GoodCRC (true: enabled).
        pub fn auto_retry(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `send_hard_reset` field of the register.
        ///
        ///Send hard reset (true: initiate, self-clearing).
        pub fn set_send_hard_reset(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `bist_tmode` field of the register.
        ///
        ///BIST test mode (true: clear RxFIFO after GoodCRC).
        pub fn set_bist_tmode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `auto_hardreset` field of the register.
        ///
        ///Auto hard reset on soft reset fail (true: enabled).
        pub fn set_auto_hardreset(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `auto_softreset` field of the register.
        ///
        ///Auto soft reset on retries fail (true: enabled).
        pub fn set_auto_softreset(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `n_retries` field of the register.
        ///
        ///Number of packet retries.
        pub fn set_n_retries(&mut self, value: super::RetryCount) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 3, &mut self.bits)
            };
        }
        ///Write the `auto_retry` field of the register.
        ///
        ///Auto retry on no GoodCRC (true: enabled).
        pub fn set_auto_retry(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Control3 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Control3> for [u8; 1] {
        fn from(val: Control3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Control3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Control3");
            {
                d.field("send_hard_reset", &self.send_hard_reset());
            }
            {
                d.field("bist_tmode", &self.bist_tmode());
            }
            {
                d.field("auto_hardreset", &self.auto_hardreset());
            }
            {
                d.field("auto_softreset", &self.auto_softreset());
            }
            {
                d.field("n_retries", &self.n_retries());
            }
            {
                d.field("auto_retry", &self.auto_retry());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Control3 { ");
            defmt::write!(f, "send_hard_reset: {=bool}, ", &self.send_hard_reset());
            defmt::write!(f, "bist_tmode: {=bool}, ", &self.bist_tmode());
            defmt::write!(f, "auto_hardreset: {=bool}, ", &self.auto_hardreset());
            defmt::write!(f, "auto_softreset: {=bool}, ", &self.auto_softreset());
            defmt::write!(f, "n_retries: {}, ", &self.n_retries());
            defmt::write!(f, "auto_retry: {=bool}, ", &self.auto_retry());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Control3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Control3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Control3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Control3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Control3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Control3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Control3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Mask Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mask {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Mask {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Mask {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `m_vbusok` field of the register.
        ///
        ///Mask I_VBUSOK interrupt (true: masked).
        pub fn m_vbusok(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `m_activity` field of the register.
        ///
        ///Mask I_ACTIVITY interrupt (true: masked).
        pub fn m_activity(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `m_comp_chng` field of the register.
        ///
        ///Mask I_COMP_CHNG interrupt (true: masked).
        pub fn m_comp_chng(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `m_crc_chk` field of the register.
        ///
        ///Mask I_CRC_CHK interrupt (true: masked).
        pub fn m_crc_chk(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `m_alert` field of the register.
        ///
        ///Mask I_ALERT interrupt (true: masked).
        pub fn m_alert(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `m_wake` field of the register.
        ///
        ///Mask I_WAKE interrupt (true: masked).
        pub fn m_wake(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `m_collision` field of the register.
        ///
        ///Mask I_COLLISION interrupt (true: masked).
        pub fn m_collision(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `m_bc_lvl` field of the register.
        ///
        ///Mask I_BC_LVL interrupt (true: masked).
        pub fn m_bc_lvl(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `m_vbusok` field of the register.
        ///
        ///Mask I_VBUSOK interrupt (true: masked).
        pub fn set_m_vbusok(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `m_activity` field of the register.
        ///
        ///Mask I_ACTIVITY interrupt (true: masked).
        pub fn set_m_activity(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `m_comp_chng` field of the register.
        ///
        ///Mask I_COMP_CHNG interrupt (true: masked).
        pub fn set_m_comp_chng(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `m_crc_chk` field of the register.
        ///
        ///Mask I_CRC_CHK interrupt (true: masked).
        pub fn set_m_crc_chk(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `m_alert` field of the register.
        ///
        ///Mask I_ALERT interrupt (true: masked).
        pub fn set_m_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `m_wake` field of the register.
        ///
        ///Mask I_WAKE interrupt (true: masked).
        pub fn set_m_wake(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `m_collision` field of the register.
        ///
        ///Mask I_COLLISION interrupt (true: masked).
        pub fn set_m_collision(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `m_bc_lvl` field of the register.
        ///
        ///Mask I_BC_LVL interrupt (true: masked).
        pub fn set_m_bc_lvl(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Mask {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Mask> for [u8; 1] {
        fn from(val: Mask) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Mask {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Mask");
            {
                d.field("m_vbusok", &self.m_vbusok());
            }
            {
                d.field("m_activity", &self.m_activity());
            }
            {
                d.field("m_comp_chng", &self.m_comp_chng());
            }
            {
                d.field("m_crc_chk", &self.m_crc_chk());
            }
            {
                d.field("m_alert", &self.m_alert());
            }
            {
                d.field("m_wake", &self.m_wake());
            }
            {
                d.field("m_collision", &self.m_collision());
            }
            {
                d.field("m_bc_lvl", &self.m_bc_lvl());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mask { ");
            defmt::write!(f, "m_vbusok: {=bool}, ", &self.m_vbusok());
            defmt::write!(f, "m_activity: {=bool}, ", &self.m_activity());
            defmt::write!(f, "m_comp_chng: {=bool}, ", &self.m_comp_chng());
            defmt::write!(f, "m_crc_chk: {=bool}, ", &self.m_crc_chk());
            defmt::write!(f, "m_alert: {=bool}, ", &self.m_alert());
            defmt::write!(f, "m_wake: {=bool}, ", &self.m_wake());
            defmt::write!(f, "m_collision: {=bool}, ", &self.m_collision());
            defmt::write!(f, "m_bc_lvl: {=bool}, ", &self.m_bc_lvl());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Mask {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Mask {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Mask {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Mask {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Mask {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Mask {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Mask {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Power Control Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Power {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Power {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pwr` field of the register.
        ///
        ///Power control bits (bitwise enables for components).
        pub fn pwr(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 4)
            };
            raw
        }
        ///Write the `pwr` field of the register.
        ///
        ///Power control bits (bitwise enables for components).
        pub fn set_pwr(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 4, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Power {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Power> for [u8; 1] {
        fn from(val: Power) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Power {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Power");
            {
                d.field("pwr", &self.pwr());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Power {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Power { ");
            defmt::write!(f, "pwr: {=u8}, ", &self.pwr());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Power {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Power {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Power {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Power {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Power {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Power {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Power {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Reset Control Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reset {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Reset {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Reset {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pd_reset` field of the register.
        ///
        ///Reset PD logic (true: reset, self-clearing).
        pub fn pd_reset(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `sw_res` field of the register.
        ///
        ///Software reset (true: reset entire device, self-clearing).
        pub fn sw_res(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `pd_reset` field of the register.
        ///
        ///Reset PD logic (true: reset, self-clearing).
        pub fn set_pd_reset(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `sw_res` field of the register.
        ///
        ///Software reset (true: reset entire device, self-clearing).
        pub fn set_sw_res(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Reset {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Reset> for [u8; 1] {
        fn from(val: Reset) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Reset {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Reset");
            {
                d.field("pd_reset", &self.pd_reset());
            }
            {
                d.field("sw_res", &self.sw_res());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reset { ");
            defmt::write!(f, "pd_reset: {=bool}, ", &self.pd_reset());
            defmt::write!(f, "sw_res: {=bool}, ", &self.sw_res());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Reset {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Reset {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Reset {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Reset {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Reset {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Reset {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Reset {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Over-Current Protection Register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ocpreg {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Ocpreg {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Ocpreg {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `ocp_range` field of the register.
        ///
        ///OCP range (true: 100-800mA, false: 10-80mA).
        pub fn ocp_range(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `ocp_cur` field of the register.
        ///
        ///OCP current setting (fraction of max range).
        pub fn ocp_cur(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 3)
            };
            raw
        }
        ///Write the `ocp_range` field of the register.
        ///
        ///OCP range (true: 100-800mA, false: 10-80mA).
        pub fn set_ocp_range(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `ocp_cur` field of the register.
        ///
        ///OCP current setting (fraction of max range).
        pub fn set_ocp_cur(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Ocpreg {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Ocpreg> for [u8; 1] {
        fn from(val: Ocpreg) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Ocpreg {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Ocpreg");
            {
                d.field("ocp_range", &self.ocp_range());
            }
            {
                d.field("ocp_cur", &self.ocp_cur());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ocpreg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ocpreg { ");
            defmt::write!(f, "ocp_range: {=bool}, ", &self.ocp_range());
            defmt::write!(f, "ocp_cur: {=u8}, ", &self.ocp_cur());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Ocpreg {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Ocpreg {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Ocpreg {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Ocpreg {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Ocpreg {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Ocpreg {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Ocpreg {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Mask Register A
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maska {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Maska {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Maska {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `m_ocp_temp` field of the register.
        ///
        ///Mask I_OCP_TEMP interrupt (true: masked).
        pub fn m_ocp_temp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `m_togdone` field of the register.
        ///
        ///Mask I_TOGDONE interrupt (true: masked).
        pub fn m_togdone(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `m_softfail` field of the register.
        ///
        ///Mask I_SOFTFAIL interrupt (true: masked).
        pub fn m_softfail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `m_retryfail` field of the register.
        ///
        ///Mask I_RETRYFAIL interrupt (true: masked).
        pub fn m_retryfail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `m_hardsent` field of the register.
        ///
        ///Mask I_HARDSENT interrupt (true: masked).
        pub fn m_hardsent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `m_txsent` field of the register.
        ///
        ///Mask I_TXSENT interrupt (true: masked).
        pub fn m_txsent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `m_softrst` field of the register.
        ///
        ///Mask I_SOFTRST interrupt (true: masked).
        pub fn m_softrst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `m_hardrst` field of the register.
        ///
        ///Mask I_HARDRST interrupt (true: masked).
        pub fn m_hardrst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `m_ocp_temp` field of the register.
        ///
        ///Mask I_OCP_TEMP interrupt (true: masked).
        pub fn set_m_ocp_temp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `m_togdone` field of the register.
        ///
        ///Mask I_TOGDONE interrupt (true: masked).
        pub fn set_m_togdone(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `m_softfail` field of the register.
        ///
        ///Mask I_SOFTFAIL interrupt (true: masked).
        pub fn set_m_softfail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `m_retryfail` field of the register.
        ///
        ///Mask I_RETRYFAIL interrupt (true: masked).
        pub fn set_m_retryfail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `m_hardsent` field of the register.
        ///
        ///Mask I_HARDSENT interrupt (true: masked).
        pub fn set_m_hardsent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `m_txsent` field of the register.
        ///
        ///Mask I_TXSENT interrupt (true: masked).
        pub fn set_m_txsent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `m_softrst` field of the register.
        ///
        ///Mask I_SOFTRST interrupt (true: masked).
        pub fn set_m_softrst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `m_hardrst` field of the register.
        ///
        ///Mask I_HARDRST interrupt (true: masked).
        pub fn set_m_hardrst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Maska {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Maska> for [u8; 1] {
        fn from(val: Maska) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Maska {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Maska");
            {
                d.field("m_ocp_temp", &self.m_ocp_temp());
            }
            {
                d.field("m_togdone", &self.m_togdone());
            }
            {
                d.field("m_softfail", &self.m_softfail());
            }
            {
                d.field("m_retryfail", &self.m_retryfail());
            }
            {
                d.field("m_hardsent", &self.m_hardsent());
            }
            {
                d.field("m_txsent", &self.m_txsent());
            }
            {
                d.field("m_softrst", &self.m_softrst());
            }
            {
                d.field("m_hardrst", &self.m_hardrst());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maska {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maska { ");
            defmt::write!(f, "m_ocp_temp: {=bool}, ", &self.m_ocp_temp());
            defmt::write!(f, "m_togdone: {=bool}, ", &self.m_togdone());
            defmt::write!(f, "m_softfail: {=bool}, ", &self.m_softfail());
            defmt::write!(f, "m_retryfail: {=bool}, ", &self.m_retryfail());
            defmt::write!(f, "m_hardsent: {=bool}, ", &self.m_hardsent());
            defmt::write!(f, "m_txsent: {=bool}, ", &self.m_txsent());
            defmt::write!(f, "m_softrst: {=bool}, ", &self.m_softrst());
            defmt::write!(f, "m_hardrst: {=bool}, ", &self.m_hardrst());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Maska {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Maska {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Maska {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Maska {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Maska {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Maska {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Maska {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Mask Register B
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maskb {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Maskb {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Maskb {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `m_gcrcsent` field of the register.
        ///
        ///Mask I_GCRCSENT interrupt (true: masked).
        pub fn m_gcrcsent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `m_gcrcsent` field of the register.
        ///
        ///Mask I_GCRCSENT interrupt (true: masked).
        pub fn set_m_gcrcsent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Maskb {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Maskb> for [u8; 1] {
        fn from(val: Maskb) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Maskb {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Maskb");
            {
                d.field("m_gcrcsent", &self.m_gcrcsent());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maskb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maskb { ");
            defmt::write!(f, "m_gcrcsent: {=bool}, ", &self.m_gcrcsent());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Maskb {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Maskb {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Maskb {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Maskb {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Maskb {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Maskb {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Maskb {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Control Register 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control4 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Control4 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Control4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `tog_exit_aud` field of the register.
        ///
        ///Toggle exit on audio accessory (true: stop on Ra both CC).
        pub fn tog_exit_aud(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `tog_exit_aud` field of the register.
        ///
        ///Toggle exit on audio accessory (true: stop on Ra both CC).
        pub fn set_tog_exit_aud(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Control4 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Control4> for [u8; 1] {
        fn from(val: Control4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Control4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Control4");
            {
                d.field("tog_exit_aud", &self.tog_exit_aud());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Control4 { ");
            defmt::write!(f, "tog_exit_aud: {=bool}, ", &self.tog_exit_aud());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Control4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Control4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Control4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Control4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Control4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Control4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Control4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Status Register 0A
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Status0A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Status0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `softfail` field of the register.
        ///
        ///Soft reset retries failed (true: failed).
        pub fn softfail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `retryfail` field of the register.
        ///
        ///Packet retries failed (true: failed).
        pub fn retryfail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `power` field of the register.
        ///
        ///Internal power state bits.
        pub fn power(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 4)
            };
            raw
        }
        ///Read the `softrst` field of the register.
        ///
        ///Soft reset packet received (true: received).
        pub fn softrst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `hardrst` field of the register.
        ///
        ///Hard reset received (true: received).
        pub fn hardrst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `softfail` field of the register.
        ///
        ///Soft reset retries failed (true: failed).
        pub fn set_softfail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `retryfail` field of the register.
        ///
        ///Packet retries failed (true: failed).
        pub fn set_retryfail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `power` field of the register.
        ///
        ///Internal power state bits.
        pub fn set_power(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `softrst` field of the register.
        ///
        ///Soft reset packet received (true: received).
        pub fn set_softrst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `hardrst` field of the register.
        ///
        ///Hard reset received (true: received).
        pub fn set_hardrst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Status0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Status0A> for [u8; 1] {
        fn from(val: Status0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Status0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Status0A");
            {
                d.field("softfail", &self.softfail());
            }
            {
                d.field("retryfail", &self.retryfail());
            }
            {
                d.field("power", &self.power());
            }
            {
                d.field("softrst", &self.softrst());
            }
            {
                d.field("hardrst", &self.hardrst());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status0A { ");
            defmt::write!(f, "softfail: {=bool}, ", &self.softfail());
            defmt::write!(f, "retryfail: {=bool}, ", &self.retryfail());
            defmt::write!(f, "power: {=u8}, ", &self.power());
            defmt::write!(f, "softrst: {=bool}, ", &self.softrst());
            defmt::write!(f, "hardrst: {=bool}, ", &self.hardrst());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Status0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Status0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Status0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Status0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Status0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Status0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Status0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Status Register 1A
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status1A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Status1A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Status1A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `togss` field of the register.
        ///
        ///Toggle state status.
        pub fn togss(
            &self,
        ) -> Result<super::ToggleState, <super::ToggleState as TryFrom<u8>>::Error> {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 6)
            };
            raw.try_into()
        }
        ///Read the `rxsop_2_db` field of the register.
        ///
        ///Last packet SOP'' Debug (true: yes).
        pub fn rxsop_2_db(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `rxsop_1_db` field of the register.
        ///
        ///Last packet SOP' Debug (true: yes).
        pub fn rxsop_1_db(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `rxsop` field of the register.
        ///
        ///Last packet SOP (true: yes).
        pub fn rxsop(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `togss` field of the register.
        ///
        ///Toggle state status.
        pub fn set_togss(&mut self, value: super::ToggleState) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 6, &mut self.bits)
            };
        }
        ///Write the `rxsop_2_db` field of the register.
        ///
        ///Last packet SOP'' Debug (true: yes).
        pub fn set_rxsop_2_db(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `rxsop_1_db` field of the register.
        ///
        ///Last packet SOP' Debug (true: yes).
        pub fn set_rxsop_1_db(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `rxsop` field of the register.
        ///
        ///Last packet SOP (true: yes).
        pub fn set_rxsop(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Status1A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Status1A> for [u8; 1] {
        fn from(val: Status1A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Status1A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Status1A");
            {
                d.field("togss", &self.togss());
            }
            {
                d.field("rxsop_2_db", &self.rxsop_2_db());
            }
            {
                d.field("rxsop_1_db", &self.rxsop_1_db());
            }
            {
                d.field("rxsop", &self.rxsop());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status1A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status1A { ");
            defmt::write!(f, "togss: {}, ", &self.togss());
            defmt::write!(f, "rxsop_2_db: {=bool}, ", &self.rxsop_2_db());
            defmt::write!(f, "rxsop_1_db: {=bool}, ", &self.rxsop_1_db());
            defmt::write!(f, "rxsop: {=bool}, ", &self.rxsop());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Status1A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Status1A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Status1A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Status1A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Status1A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Status1A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Status1A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Register A (read/clear)
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Interrupta {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Interrupta {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Interrupta {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `i_ocp_temp` field of the register.
        ///
        ///OCP or over-temp interrupt (true: occurred).
        pub fn i_ocp_temp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `i_togdone` field of the register.
        ///
        ///Toggle done interrupt (true: occurred).
        pub fn i_togdone(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `i_softfail` field of the register.
        ///
        ///Soft reset fail interrupt (true: occurred).
        pub fn i_softfail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `i_retryfail` field of the register.
        ///
        ///Retry fail interrupt (true: occurred).
        pub fn i_retryfail(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `i_hardsent` field of the register.
        ///
        ///Hard reset sent interrupt (true: occurred).
        pub fn i_hardsent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `i_txsent` field of the register.
        ///
        ///Packet sent interrupt (true: occurred).
        pub fn i_txsent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `i_softrst` field of the register.
        ///
        ///Soft reset received interrupt (true: occurred).
        pub fn i_softrst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `i_hardrst` field of the register.
        ///
        ///Hard reset received interrupt (true: occurred).
        pub fn i_hardrst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `i_ocp_temp` field of the register.
        ///
        ///OCP or over-temp interrupt (true: occurred).
        pub fn set_i_ocp_temp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `i_togdone` field of the register.
        ///
        ///Toggle done interrupt (true: occurred).
        pub fn set_i_togdone(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `i_softfail` field of the register.
        ///
        ///Soft reset fail interrupt (true: occurred).
        pub fn set_i_softfail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `i_retryfail` field of the register.
        ///
        ///Retry fail interrupt (true: occurred).
        pub fn set_i_retryfail(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `i_hardsent` field of the register.
        ///
        ///Hard reset sent interrupt (true: occurred).
        pub fn set_i_hardsent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `i_txsent` field of the register.
        ///
        ///Packet sent interrupt (true: occurred).
        pub fn set_i_txsent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `i_softrst` field of the register.
        ///
        ///Soft reset received interrupt (true: occurred).
        pub fn set_i_softrst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `i_hardrst` field of the register.
        ///
        ///Hard reset received interrupt (true: occurred).
        pub fn set_i_hardrst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Interrupta {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Interrupta> for [u8; 1] {
        fn from(val: Interrupta) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Interrupta {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Interrupta");
            {
                d.field("i_ocp_temp", &self.i_ocp_temp());
            }
            {
                d.field("i_togdone", &self.i_togdone());
            }
            {
                d.field("i_softfail", &self.i_softfail());
            }
            {
                d.field("i_retryfail", &self.i_retryfail());
            }
            {
                d.field("i_hardsent", &self.i_hardsent());
            }
            {
                d.field("i_txsent", &self.i_txsent());
            }
            {
                d.field("i_softrst", &self.i_softrst());
            }
            {
                d.field("i_hardrst", &self.i_hardrst());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Interrupta {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Interrupta { ");
            defmt::write!(f, "i_ocp_temp: {=bool}, ", &self.i_ocp_temp());
            defmt::write!(f, "i_togdone: {=bool}, ", &self.i_togdone());
            defmt::write!(f, "i_softfail: {=bool}, ", &self.i_softfail());
            defmt::write!(f, "i_retryfail: {=bool}, ", &self.i_retryfail());
            defmt::write!(f, "i_hardsent: {=bool}, ", &self.i_hardsent());
            defmt::write!(f, "i_txsent: {=bool}, ", &self.i_txsent());
            defmt::write!(f, "i_softrst: {=bool}, ", &self.i_softrst());
            defmt::write!(f, "i_hardrst: {=bool}, ", &self.i_hardrst());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Interrupta {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Interrupta {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Interrupta {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Interrupta {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Interrupta {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Interrupta {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Interrupta {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Register B (read/clear)
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Interruptb {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Interruptb {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Interruptb {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `i_gcrcsent` field of the register.
        ///
        ///GoodCRC sent interrupt (true: occurred).
        pub fn i_gcrcsent(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `i_gcrcsent` field of the register.
        ///
        ///GoodCRC sent interrupt (true: occurred).
        pub fn set_i_gcrcsent(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Interruptb {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Interruptb> for [u8; 1] {
        fn from(val: Interruptb) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Interruptb {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Interruptb");
            {
                d.field("i_gcrcsent", &self.i_gcrcsent());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Interruptb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Interruptb { ");
            defmt::write!(f, "i_gcrcsent: {=bool}, ", &self.i_gcrcsent());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Interruptb {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Interruptb {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Interruptb {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Interruptb {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Interruptb {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Interruptb {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Interruptb {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Status Register 0
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status0 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Status0 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Status0 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `vbusok` field of the register.
        ///
        ///VBUS OK (true: VBUS valid).
        pub fn vbusok(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `activity` field of the register.
        ///
        ///CC activity detected (true: active).
        pub fn activity(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `comp` field of the register.
        ///
        ///Comparator result (true: CC > MDAC).
        pub fn comp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `crc_chk` field of the register.
        ///
        ///CRC check valid (true: last packet CRC correct).
        pub fn crc_chk(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `alert` field of the register.
        ///
        ///Alert condition (true: error occurred).
        pub fn alert(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `wake` field of the register.
        ///
        ///Wake detected (true: device attempting attach).
        pub fn wake(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `bc_lvl` field of the register.
        ///
        ///Battery charge level on CC.
        pub fn bc_lvl(&self) -> super::BcLvl {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `vbusok` field of the register.
        ///
        ///VBUS OK (true: VBUS valid).
        pub fn set_vbusok(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `activity` field of the register.
        ///
        ///CC activity detected (true: active).
        pub fn set_activity(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `comp` field of the register.
        ///
        ///Comparator result (true: CC > MDAC).
        pub fn set_comp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `crc_chk` field of the register.
        ///
        ///CRC check valid (true: last packet CRC correct).
        pub fn set_crc_chk(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `alert` field of the register.
        ///
        ///Alert condition (true: error occurred).
        pub fn set_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `wake` field of the register.
        ///
        ///Wake detected (true: device attempting attach).
        pub fn set_wake(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `bc_lvl` field of the register.
        ///
        ///Battery charge level on CC.
        pub fn set_bc_lvl(&mut self, value: super::BcLvl) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Status0 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Status0> for [u8; 1] {
        fn from(val: Status0) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Status0 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Status0");
            {
                d.field("vbusok", &self.vbusok());
            }
            {
                d.field("activity", &self.activity());
            }
            {
                d.field("comp", &self.comp());
            }
            {
                d.field("crc_chk", &self.crc_chk());
            }
            {
                d.field("alert", &self.alert());
            }
            {
                d.field("wake", &self.wake());
            }
            {
                d.field("bc_lvl", &self.bc_lvl());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status0 { ");
            defmt::write!(f, "vbusok: {=bool}, ", &self.vbusok());
            defmt::write!(f, "activity: {=bool}, ", &self.activity());
            defmt::write!(f, "comp: {=bool}, ", &self.comp());
            defmt::write!(f, "crc_chk: {=bool}, ", &self.crc_chk());
            defmt::write!(f, "alert: {=bool}, ", &self.alert());
            defmt::write!(f, "wake: {=bool}, ", &self.wake());
            defmt::write!(f, "bc_lvl: {}, ", &self.bc_lvl());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Status0 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Status0 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Status0 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Status0 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Status0 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Status0 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Status0 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Status Register 1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Status1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Status1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `rxsop_2` field of the register.
        ///
        ///Last packet SOP'' (true: yes).
        pub fn rxsop_2(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `rxsop_1` field of the register.
        ///
        ///Last packet SOP' (true: yes).
        pub fn rxsop_1(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `rx_empty` field of the register.
        ///
        ///Receive FIFO empty (true: empty).
        pub fn rx_empty(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `rx_full` field of the register.
        ///
        ///Receive FIFO full (true: full).
        pub fn rx_full(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `tx_empty` field of the register.
        ///
        ///Transmit FIFO empty (true: empty).
        pub fn tx_empty(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `tx_full` field of the register.
        ///
        ///Transmit FIFO full (true: full).
        pub fn tx_full(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `ovrtemp` field of the register.
        ///
        ///Over-temperature (true: too high).
        pub fn ovrtemp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `ocp` field of the register.
        ///
        ///Over-current on VCONN (true: occurred).
        pub fn ocp(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `rxsop_2` field of the register.
        ///
        ///Last packet SOP'' (true: yes).
        pub fn set_rxsop_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `rxsop_1` field of the register.
        ///
        ///Last packet SOP' (true: yes).
        pub fn set_rxsop_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `rx_empty` field of the register.
        ///
        ///Receive FIFO empty (true: empty).
        pub fn set_rx_empty(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `rx_full` field of the register.
        ///
        ///Receive FIFO full (true: full).
        pub fn set_rx_full(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `tx_empty` field of the register.
        ///
        ///Transmit FIFO empty (true: empty).
        pub fn set_tx_empty(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `tx_full` field of the register.
        ///
        ///Transmit FIFO full (true: full).
        pub fn set_tx_full(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `ovrtemp` field of the register.
        ///
        ///Over-temperature (true: too high).
        pub fn set_ovrtemp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `ocp` field of the register.
        ///
        ///Over-current on VCONN (true: occurred).
        pub fn set_ocp(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Status1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Status1> for [u8; 1] {
        fn from(val: Status1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Status1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Status1");
            {
                d.field("rxsop_2", &self.rxsop_2());
            }
            {
                d.field("rxsop_1", &self.rxsop_1());
            }
            {
                d.field("rx_empty", &self.rx_empty());
            }
            {
                d.field("rx_full", &self.rx_full());
            }
            {
                d.field("tx_empty", &self.tx_empty());
            }
            {
                d.field("tx_full", &self.tx_full());
            }
            {
                d.field("ovrtemp", &self.ovrtemp());
            }
            {
                d.field("ocp", &self.ocp());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status1 { ");
            defmt::write!(f, "rxsop_2: {=bool}, ", &self.rxsop_2());
            defmt::write!(f, "rxsop_1: {=bool}, ", &self.rxsop_1());
            defmt::write!(f, "rx_empty: {=bool}, ", &self.rx_empty());
            defmt::write!(f, "rx_full: {=bool}, ", &self.rx_full());
            defmt::write!(f, "tx_empty: {=bool}, ", &self.tx_empty());
            defmt::write!(f, "tx_full: {=bool}, ", &self.tx_full());
            defmt::write!(f, "ovrtemp: {=bool}, ", &self.ovrtemp());
            defmt::write!(f, "ocp: {=bool}, ", &self.ocp());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Status1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Status1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Status1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Status1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Status1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Status1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Status1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Status Register (read/clear)
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Interrupt {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Interrupt {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Interrupt {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `i_vbusok` field of the register.
        ///
        ///VBUSOK interrupt (true: VBUS transitioned).
        pub fn i_vbusok(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `i_activity` field of the register.
        ///
        ///Activity interrupt (true: CC activity change).
        pub fn i_activity(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `i_comp_chng` field of the register.
        ///
        ///Comparator change interrupt (true: occurred).
        pub fn i_comp_chng(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `i_crc_chk` field of the register.
        ///
        ///CRC check interrupt (true: packet validated).
        pub fn i_crc_chk(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `i_alert` field of the register.
        ///
        ///Alert interrupt (true: error condition).
        pub fn i_alert(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `i_wake` field of the register.
        ///
        ///Wake interrupt (true: device attach attempt).
        pub fn i_wake(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `i_collision` field of the register.
        ///
        ///Collision interrupt (true: transmit attempted during activity).
        pub fn i_collision(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `i_bc_lvl` field of the register.
        ///
        ///BC_LVL interrupt (true: current level changed).
        pub fn i_bc_lvl(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `i_vbusok` field of the register.
        ///
        ///VBUSOK interrupt (true: VBUS transitioned).
        pub fn set_i_vbusok(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `i_activity` field of the register.
        ///
        ///Activity interrupt (true: CC activity change).
        pub fn set_i_activity(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `i_comp_chng` field of the register.
        ///
        ///Comparator change interrupt (true: occurred).
        pub fn set_i_comp_chng(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `i_crc_chk` field of the register.
        ///
        ///CRC check interrupt (true: packet validated).
        pub fn set_i_crc_chk(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `i_alert` field of the register.
        ///
        ///Alert interrupt (true: error condition).
        pub fn set_i_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `i_wake` field of the register.
        ///
        ///Wake interrupt (true: device attach attempt).
        pub fn set_i_wake(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `i_collision` field of the register.
        ///
        ///Collision interrupt (true: transmit attempted during activity).
        pub fn set_i_collision(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `i_bc_lvl` field of the register.
        ///
        ///BC_LVL interrupt (true: current level changed).
        pub fn set_i_bc_lvl(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Interrupt {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Interrupt> for [u8; 1] {
        fn from(val: Interrupt) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Interrupt {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Interrupt");
            {
                d.field("i_vbusok", &self.i_vbusok());
            }
            {
                d.field("i_activity", &self.i_activity());
            }
            {
                d.field("i_comp_chng", &self.i_comp_chng());
            }
            {
                d.field("i_crc_chk", &self.i_crc_chk());
            }
            {
                d.field("i_alert", &self.i_alert());
            }
            {
                d.field("i_wake", &self.i_wake());
            }
            {
                d.field("i_collision", &self.i_collision());
            }
            {
                d.field("i_bc_lvl", &self.i_bc_lvl());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Interrupt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Interrupt { ");
            defmt::write!(f, "i_vbusok: {=bool}, ", &self.i_vbusok());
            defmt::write!(f, "i_activity: {=bool}, ", &self.i_activity());
            defmt::write!(f, "i_comp_chng: {=bool}, ", &self.i_comp_chng());
            defmt::write!(f, "i_crc_chk: {=bool}, ", &self.i_crc_chk());
            defmt::write!(f, "i_alert: {=bool}, ", &self.i_alert());
            defmt::write!(f, "i_wake: {=bool}, ", &self.i_wake());
            defmt::write!(f, "i_collision: {=bool}, ", &self.i_collision());
            defmt::write!(f, "i_bc_lvl: {=bool}, ", &self.i_bc_lvl());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Interrupt {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Interrupt {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Interrupt {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Interrupt {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Interrupt {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Interrupt {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Interrupt {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        ///Device Identification Register
        DeviceId(DeviceId),
        ///Switch Control Register 0
        Switches0(Switches0),
        ///Switch Control Register 1
        Switches1(Switches1),
        ///Measure Control Register
        Measure(Measure),
        ///Slice Control Register
        Slice(Slice),
        ///Control Register 0
        Control0(Control0),
        ///Control Register 1
        Control1(Control1),
        ///Control Register 2
        Control2(Control2),
        ///Control Register 3
        Control3(Control3),
        ///Interrupt Mask Register
        Mask(Mask),
        ///Power Control Register
        Power(Power),
        ///Reset Control Register
        Reset(Reset),
        ///Over-Current Protection Register
        Ocpreg(Ocpreg),
        ///Interrupt Mask Register A
        Maska(Maska),
        ///Interrupt Mask Register B
        Maskb(Maskb),
        ///Control Register 4
        Control4(Control4),
        ///Status Register 0A
        Status0A(Status0A),
        ///Status Register 1A
        Status1A(Status1A),
        ///Interrupt Register A (read/clear)
        Interrupta(Interrupta),
        ///Interrupt Register B (read/clear)
        Interruptb(Interruptb),
        ///Status Register 0
        Status0(Status0),
        ///Status Register 1
        Status1(Status1),
        ///Interrupt Status Register (read/clear)
        Interrupt(Interrupt),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::DeviceId(val) => core::fmt::Debug::fmt(val, f),
                Self::Switches0(val) => core::fmt::Debug::fmt(val, f),
                Self::Switches1(val) => core::fmt::Debug::fmt(val, f),
                Self::Measure(val) => core::fmt::Debug::fmt(val, f),
                Self::Slice(val) => core::fmt::Debug::fmt(val, f),
                Self::Control0(val) => core::fmt::Debug::fmt(val, f),
                Self::Control1(val) => core::fmt::Debug::fmt(val, f),
                Self::Control2(val) => core::fmt::Debug::fmt(val, f),
                Self::Control3(val) => core::fmt::Debug::fmt(val, f),
                Self::Mask(val) => core::fmt::Debug::fmt(val, f),
                Self::Power(val) => core::fmt::Debug::fmt(val, f),
                Self::Reset(val) => core::fmt::Debug::fmt(val, f),
                Self::Ocpreg(val) => core::fmt::Debug::fmt(val, f),
                Self::Maska(val) => core::fmt::Debug::fmt(val, f),
                Self::Maskb(val) => core::fmt::Debug::fmt(val, f),
                Self::Control4(val) => core::fmt::Debug::fmt(val, f),
                Self::Status0A(val) => core::fmt::Debug::fmt(val, f),
                Self::Status1A(val) => core::fmt::Debug::fmt(val, f),
                Self::Interrupta(val) => core::fmt::Debug::fmt(val, f),
                Self::Interruptb(val) => core::fmt::Debug::fmt(val, f),
                Self::Status0(val) => core::fmt::Debug::fmt(val, f),
                Self::Status1(val) => core::fmt::Debug::fmt(val, f),
                Self::Interrupt(val) => core::fmt::Debug::fmt(val, f),
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::DeviceId(val) => defmt::Format::format(val, f),
                Self::Switches0(val) => defmt::Format::format(val, f),
                Self::Switches1(val) => defmt::Format::format(val, f),
                Self::Measure(val) => defmt::Format::format(val, f),
                Self::Slice(val) => defmt::Format::format(val, f),
                Self::Control0(val) => defmt::Format::format(val, f),
                Self::Control1(val) => defmt::Format::format(val, f),
                Self::Control2(val) => defmt::Format::format(val, f),
                Self::Control3(val) => defmt::Format::format(val, f),
                Self::Mask(val) => defmt::Format::format(val, f),
                Self::Power(val) => defmt::Format::format(val, f),
                Self::Reset(val) => defmt::Format::format(val, f),
                Self::Ocpreg(val) => defmt::Format::format(val, f),
                Self::Maska(val) => defmt::Format::format(val, f),
                Self::Maskb(val) => defmt::Format::format(val, f),
                Self::Control4(val) => defmt::Format::format(val, f),
                Self::Status0A(val) => defmt::Format::format(val, f),
                Self::Status1A(val) => defmt::Format::format(val, f),
                Self::Interrupta(val) => defmt::Format::format(val, f),
                Self::Interruptb(val) => defmt::Format::format(val, f),
                Self::Status0(val) => defmt::Format::format(val, f),
                Self::Status1(val) => defmt::Format::format(val, f),
                Self::Interrupt(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<DeviceId> for FieldSetValue {
        fn from(val: DeviceId) -> Self {
            Self::DeviceId(val)
        }
    }
    impl From<Switches0> for FieldSetValue {
        fn from(val: Switches0) -> Self {
            Self::Switches0(val)
        }
    }
    impl From<Switches1> for FieldSetValue {
        fn from(val: Switches1) -> Self {
            Self::Switches1(val)
        }
    }
    impl From<Measure> for FieldSetValue {
        fn from(val: Measure) -> Self {
            Self::Measure(val)
        }
    }
    impl From<Slice> for FieldSetValue {
        fn from(val: Slice) -> Self {
            Self::Slice(val)
        }
    }
    impl From<Control0> for FieldSetValue {
        fn from(val: Control0) -> Self {
            Self::Control0(val)
        }
    }
    impl From<Control1> for FieldSetValue {
        fn from(val: Control1) -> Self {
            Self::Control1(val)
        }
    }
    impl From<Control2> for FieldSetValue {
        fn from(val: Control2) -> Self {
            Self::Control2(val)
        }
    }
    impl From<Control3> for FieldSetValue {
        fn from(val: Control3) -> Self {
            Self::Control3(val)
        }
    }
    impl From<Mask> for FieldSetValue {
        fn from(val: Mask) -> Self {
            Self::Mask(val)
        }
    }
    impl From<Power> for FieldSetValue {
        fn from(val: Power) -> Self {
            Self::Power(val)
        }
    }
    impl From<Reset> for FieldSetValue {
        fn from(val: Reset) -> Self {
            Self::Reset(val)
        }
    }
    impl From<Ocpreg> for FieldSetValue {
        fn from(val: Ocpreg) -> Self {
            Self::Ocpreg(val)
        }
    }
    impl From<Maska> for FieldSetValue {
        fn from(val: Maska) -> Self {
            Self::Maska(val)
        }
    }
    impl From<Maskb> for FieldSetValue {
        fn from(val: Maskb) -> Self {
            Self::Maskb(val)
        }
    }
    impl From<Control4> for FieldSetValue {
        fn from(val: Control4) -> Self {
            Self::Control4(val)
        }
    }
    impl From<Status0A> for FieldSetValue {
        fn from(val: Status0A) -> Self {
            Self::Status0A(val)
        }
    }
    impl From<Status1A> for FieldSetValue {
        fn from(val: Status1A) -> Self {
            Self::Status1A(val)
        }
    }
    impl From<Interrupta> for FieldSetValue {
        fn from(val: Interrupta) -> Self {
            Self::Interrupta(val)
        }
    }
    impl From<Interruptb> for FieldSetValue {
        fn from(val: Interruptb) -> Self {
            Self::Interruptb(val)
        }
    }
    impl From<Status0> for FieldSetValue {
        fn from(val: Status0) -> Self {
            Self::Status0(val)
        }
    }
    impl From<Status1> for FieldSetValue {
        fn from(val: Status1) -> Self {
            Self::Status1(val)
        }
    }
    impl From<Interrupt> for FieldSetValue {
        fn from(val: Interrupt) -> Self {
            Self::Interrupt(val)
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpecRev {
    ///Revision 1.0
    Revision10 = 0,
    ///Revision 2.0
    Revision20 = 1,
}
impl core::convert::TryFrom<u8> for SpecRev {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Revision10),
            1 => Ok(Self::Revision20),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "SpecRev",
                })
            }
        }
    }
}
impl From<SpecRev> for u8 {
    fn from(val: SpecRev) -> Self {
        match val {
            SpecRev::Revision10 => 0,
            SpecRev::Revision20 => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdacHys {
    ///No hysteresis
    NoHysteresis = 0,
    ///85mV hysteresis
    Hys85MV = 1,
    ///170mV hysteresis
    Hys170MV = 2,
    ///255mV hysteresis
    Hys255MV = 3,
}
impl core::convert::TryFrom<u8> for SdacHys {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoHysteresis),
            1 => Ok(Self::Hys85MV),
            2 => Ok(Self::Hys170MV),
            3 => Ok(Self::Hys255MV),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "SdacHys",
                })
            }
        }
    }
}
impl From<SdacHys> for u8 {
    fn from(val: SdacHys) -> Self {
        match val {
            SdacHys::NoHysteresis => 0,
            SdacHys::Hys85MV => 1,
            SdacHys::Hys170MV => 2,
            SdacHys::Hys255MV => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostCurrent {
    ///No current
    NoCurrent = 0,
    ///80µA - Default USB power
    Default80UA = 1,
    ///180µA - 1.5A
    Medium180UA = 2,
    ///330µA - 3A
    High330UA = 3,
}
impl core::convert::TryFrom<u8> for HostCurrent {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoCurrent),
            1 => Ok(Self::Default80UA),
            2 => Ok(Self::Medium180UA),
            3 => Ok(Self::High330UA),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "HostCurrent",
                })
            }
        }
    }
}
impl From<HostCurrent> for u8 {
    fn from(val: HostCurrent) -> Self {
        match val {
            HostCurrent::NoCurrent => 0,
            HostCurrent::Default80UA => 1,
            HostCurrent::Medium180UA => 2,
            HostCurrent::High330UA => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TogSavePwr {
    ///No disable state
    NoDisable = 0,
    ///Wait 40ms between cycles
    Wait40Ms = 1,
    ///Wait 80ms between cycles
    Wait80Ms = 2,
    ///Wait 160ms between cycles
    Wait160Ms = 3,
}
impl core::convert::TryFrom<u8> for TogSavePwr {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoDisable),
            1 => Ok(Self::Wait40Ms),
            2 => Ok(Self::Wait80Ms),
            3 => Ok(Self::Wait160Ms),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "TogSavePwr",
                })
            }
        }
    }
}
impl From<TogSavePwr> for u8 {
    fn from(val: TogSavePwr) -> Self {
        match val {
            TogSavePwr::NoDisable => 0,
            TogSavePwr::Wait40Ms => 1,
            TogSavePwr::Wait80Ms => 2,
            TogSavePwr::Wait160Ms => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ToggleMode {
    ///SRC polling
    SrcPolling = 3,
    ///SNK polling
    SnkPolling = 2,
    ///DRP polling
    DrpPolling = 1,
}
impl core::convert::TryFrom<u8> for ToggleMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            3 => Ok(Self::SrcPolling),
            2 => Ok(Self::SnkPolling),
            1 => Ok(Self::DrpPolling),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ToggleMode",
                })
            }
        }
    }
}
impl From<ToggleMode> for u8 {
    fn from(val: ToggleMode) -> Self {
        match val {
            ToggleMode::SrcPolling => 3,
            ToggleMode::SnkPolling => 2,
            ToggleMode::DrpPolling => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RetryCount {
    ///No retries
    NoRetries = 0,
    ///1 retry
    OneRetry = 1,
    ///2 retries
    TwoRetries = 2,
    ///3 retries
    ThreeRetries = 3,
}
impl core::convert::TryFrom<u8> for RetryCount {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoRetries),
            1 => Ok(Self::OneRetry),
            2 => Ok(Self::TwoRetries),
            3 => Ok(Self::ThreeRetries),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "RetryCount",
                })
            }
        }
    }
}
impl From<RetryCount> for u8 {
    fn from(val: RetryCount) -> Self {
        match val {
            RetryCount::NoRetries => 0,
            RetryCount::OneRetry => 1,
            RetryCount::TwoRetries => 2,
            RetryCount::ThreeRetries => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ToggleState {
    ///Toggle logic running
    Running = 0,
    ///Settled to SRC on CC1
    SrcCc1 = 1,
    ///Settled to SRC on CC2
    SrcCc2 = 2,
    ///Settled to SNK on CC1
    SnkCc1 = 5,
    ///Settled to SNK on CC2
    SnkCc2 = 6,
    ///Audio accessory detected
    AudioAccessory = 7,
}
impl core::convert::TryFrom<u8> for ToggleState {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Running),
            1 => Ok(Self::SrcCc1),
            2 => Ok(Self::SrcCc2),
            5 => Ok(Self::SnkCc1),
            6 => Ok(Self::SnkCc2),
            7 => Ok(Self::AudioAccessory),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ToggleState",
                })
            }
        }
    }
}
impl From<ToggleState> for u8 {
    fn from(val: ToggleState) -> Self {
        match val {
            ToggleState::Running => 0,
            ToggleState::SrcCc1 => 1,
            ToggleState::SrcCc2 => 2,
            ToggleState::SnkCc1 => 5,
            ToggleState::SnkCc2 => 6,
            ToggleState::AudioAccessory => 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BcLvl {
    ///<200mV
    LessThan200MV = 0,
    ///>200mV, <660mV
    Between200And660MV = 1,
    ///>660mV, <1.23V
    Between660And1230MV = 2,
    ///>1.23V
    GreaterThan1230MV = 3,
}
impl core::convert::TryFrom<u8> for BcLvl {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::LessThan200MV),
            1 => Ok(Self::Between200And660MV),
            2 => Ok(Self::Between660And1230MV),
            3 => Ok(Self::GreaterThan1230MV),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "BcLvl",
                })
            }
        }
    }
}
impl From<BcLvl> for u8 {
    fn from(val: BcLvl) -> Self {
        match val {
            BcLvl::LessThan200MV => 0,
            BcLvl::Between200And660MV => 1,
            BcLvl::Between660And1230MV => 2,
            BcLvl::GreaterThan1230MV => 3,
        }
    }
}
