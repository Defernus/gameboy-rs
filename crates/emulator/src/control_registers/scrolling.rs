use crate::*;

/// SCY: Background viewport Y position
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF42)]
pub struct RegisterSCY(pub u8);

/// SCX: Background viewport X position
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF43)]
pub struct RegisterSCX(pub u8);

/// WY: Window Y position
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF4A)]
pub struct RegisterWY(pub u8);

/// WX: Window X position
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF4B)]
pub struct RegisterWX(pub u8);
