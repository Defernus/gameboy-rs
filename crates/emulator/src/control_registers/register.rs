use crate::*;

/// Register from I/O memory range
pub trait ControlRegister: From<u8> + Into<u8> {
    const ADDRESS: u16;

    fn from_memory_mut(emulator: &mut Emulator) -> &mut Self;
}

pub trait BitFlagRegister: ControlRegister {
    fn set_flag(&mut self, flag: u8, value: bool);
    fn get_flag(&self, flag: u8) -> bool;
}
