use crate::*;
use bit_flag::{bit_flag, flag_mask};

/// Interrupt enable
#[derive(Default, Copy, Clone, ControlRegister)]
#[register(address = 0xFF40)]
pub struct RegisterIE(pub u8);

#[bit_flag]
impl RegisterIE {
    /// Controls whether the VBlank interrupt handler may be called.
    #[flag_mask]
    pub const V_BLANK: u8 = 0b0000_0001;

    /// Controls whether the LCD interrupt handler may be called.
    #[flag_mask]
    pub const LCD: u8 = 0b0000_0010;

    /// Controls whether the Timer interrupt handler may be called.
    #[flag_mask]
    pub const TIMER: u8 = 0b0000_0100;

    /// Controls whether the Serial interrupt handler may be called.
    #[flag_mask]
    pub const SERIAL: u8 = 0b0000_1000;

    /// Controls whether the Joypad interrupt handler may be called.
    #[flag_mask]
    pub const JOYPAD: u8 = 0b0001_0000;
}

/// Interrupt flag
#[derive(Default, Copy, Clone, ControlRegister)]
#[register(address = 0xFF40)]
pub struct RegisterIF(pub u8);

#[bit_flag]
impl RegisterIF {
    /// Controls whether the VBlank interrupt handler is being requested.
    #[flag_mask]
    pub const V_BLANK: u8 = 0b0000_0001;

    /// Controls whether the LCD interrupt handler is being requested.
    #[flag_mask]
    pub const LCD: u8 = 0b0000_0010;

    /// Controls whether the Timer interrupt handler is being requested.
    #[flag_mask]
    pub const TIMER: u8 = 0b0000_0100;

    /// Controls whether the Serial interrupt handler is being requested.
    #[flag_mask]
    pub const SERIAL: u8 = 0b0000_1000;

    /// Controls whether the Joypad interrupt handler is being requested.
    #[flag_mask]
    pub const JOYPAD: u8 = 0b0001_0000;
}
