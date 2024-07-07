use emulator_derive::ControlRegister;

/// Timer counter
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF44)]
pub struct RegisterTIMA(pub u8);
