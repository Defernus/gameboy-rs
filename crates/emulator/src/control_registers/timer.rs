use crate::*;
use bit_flag::{bit_flag, flag_mask, value_mask, U2};
use emulator_derive::ControlRegister;

///
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF04)]
pub struct RegisterDIV(pub u8);

/// Timer counter:  
/// This timer is incremented by a clock frequency specified by the TAC register
/// ($FF07). When the value overflows then it will be reset to the value specified
/// in TMA ($FF06), and an interrupt will be requested, as described below.
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF05)]
pub struct RegisterTIMA(pub u8);

/// Timer Modulo:  
/// When the TIMA overflows, this data will be loaded.
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF06)]
pub struct RegisterTMA(pub u8);

/// Timer Control
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF06)]
pub struct RegisterTAC(pub u8);

#[bit_flag]
impl RegisterTAC {
    #[flag_mask]
    pub const ENABLE: u8 = 0b0000_0100;

    #[value_mask(ClockMode)]
    pub const CLOCK_MODE: u8 = 0b0000_0011;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum ClockMode {
    Every256,
    Every4,
    Every16,
    Every64,
}

impl From<U2> for ClockMode {
    fn from(value: U2) -> Self {
        match value.to_bits() {
            [false, false] => ClockMode::Every256,
            [true, false] => ClockMode::Every4,
            [false, true] => ClockMode::Every16,
            [true, true] => ClockMode::Every64,
        }
    }
}

impl From<ClockMode> for U2 {
    fn from(value: ClockMode) -> Self {
        match value {
            ClockMode::Every256 => U2::new(0),
            ClockMode::Every4 => U2::new(1),
            ClockMode::Every16 => U2::new(2),
            ClockMode::Every64 => U2::new(3),
        }
    }
}

impl ClockMode {
    /// Get the bit which will effect clock speed
    pub fn get_bit_shift(self) -> u8 {
        match self {
            ClockMode::Every256 => 7,
            ClockMode::Every4 => 1,
            ClockMode::Every16 => 3,
            ClockMode::Every64 => 5,
        }
    }
}
impl Emulator {
    pub fn tick_internal_timer(&mut self) {
        let timer_state = self.internal_timer.as_u16();

        *self.internal_timer.as_u16_mut() = timer_state.wrapping_add(1);

        let new_timer_state = self.internal_timer.as_u16();

        let tac = self.reg::<RegisterTAC>();

        if !tac.get_enable() {
            return;
        }

        let mode = tac.get_clock_mode();
        let shift = mode.get_bit_shift();

        if (new_timer_state >> shift) > (timer_state >> shift) {
            self.tick_tima();
        }
    }

    fn tick_tima(&mut self) {
        let tima = self.reg::<RegisterTIMA>().0;

        if tima == 0xFF {
            self.reg_mut::<RegisterTIMA>().0 = self.reg::<RegisterTMA>().0;
            self.reg_mut::<RegisterIF>().set_timer(true);
        } else {
            self.reg_mut::<RegisterTIMA>().increment();
        }
    }
}
