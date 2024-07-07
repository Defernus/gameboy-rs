use crate::*;

/// Register from I/O memory range
pub trait ControlRegister: From<u8> + Into<u8> + Copy {
    const ADDRESS: u16;

    /// Get mutable reference to control register
    fn from_memory_mut(emulator: &mut Emulator) -> &mut Self;
    fn from_memory(emulator: &Emulator) -> &Self;

    fn increment(&mut self) {
        *self = Into::<u8>::into(*self).wrapping_add(1).into();
    }
}

impl Emulator {
    /// Get control register
    pub fn reg<T: ControlRegister>(&self) -> &T {
        debug_assert!(
            MEMORY_RANGE_IO_REGISTERS.contains(&(T::ADDRESS as usize)),
            "Control register out of IO memory range: {:04X}",
            T::ADDRESS
        );

        T::from_memory(self)
    }

    /// Get mutable reference to control register
    pub fn reg_mut<T: ControlRegister>(&mut self) -> &mut T {
        debug_assert!(
            MEMORY_RANGE_IO_REGISTERS.contains(&(T::ADDRESS as usize)),
            "Control register out of IO memory range: {:04X}",
            T::ADDRESS
        );

        T::from_memory_mut(self)
    }

    /// Reset register to default value
    pub fn reg_reset<T: ControlRegister + Default>(&mut self) {
        *self.reg_mut::<T>() = T::default();
    }
}
