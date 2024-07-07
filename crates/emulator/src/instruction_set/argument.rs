use bit_flag::U3;

use crate::*;

/// r8 argument type. Any of the 8-bit registers or memory at HL.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ArgumentR8 {
    B,
    C,
    D,
    E,
    H,
    L,
    AtHL,
    A,
}

impl ArgumentR8 {
    /// Used to parse the bits of the opcode to determine the register
    pub fn from_bits(b0: bool, b1: bool, b2: bool) -> Self {
        match (b0, b1, b2) {
            (false, false, false) => Self::B,
            (true, false, false) => Self::C,
            (false, true, false) => Self::D,
            (true, true, false) => Self::E,
            (false, false, true) => Self::H,
            (true, false, true) => Self::L,
            (false, true, true) => Self::AtHL,
            (true, true, true) => Self::A,
        }
    }
}

impl ArgumentWrite for ArgumentR8 {
    type Value = u8;

    fn set(&self, emulator: &mut Emulator, value: Self::Value) {
        match self {
            Self::B => emulator.register_bc.set_high(value),
            Self::C => emulator.register_bc.set_low(value),
            Self::D => emulator.register_de.set_high(value),
            Self::E => emulator.register_de.set_low(value),
            Self::H => emulator.register_hl.set_high(value),
            Self::L => emulator.register_hl.set_low(value),
            Self::AtHL => emulator.set(emulator.register_hl.as_u16(), value),
            Self::A => emulator.accumulator_and_flags.set_high(value),
        }
    }
}

impl ArgumentRead for ArgumentR8 {
    type Result = u8;

    /// Returns the value of the 8-bit register or memory at HL.
    fn get(&self, emulator: &Emulator) -> Self::Result {
        match self {
            Self::B => emulator.register_bc.high(),
            Self::C => emulator.register_bc.low(),
            Self::D => emulator.register_de.high(),
            Self::E => emulator.register_de.low(),
            Self::H => emulator.register_hl.high(),
            Self::L => emulator.register_hl.low(),
            Self::AtHL => emulator.get(emulator.register_hl.as_u16()),
            Self::A => emulator.accumulator_and_flags.high(),
        }
    }
}

/// r16 argument. Any of the general-purpose 16-bit registers and SP (stack pointer).
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ArgumentR16 {
    BC,
    DE,
    HL,
    SP,
}

impl ArgumentR16 {
    /// Used to parse the bits of the opcode to determine the register
    pub fn from_bits(b0: bool, b1: bool) -> Self {
        match (b0, b1) {
            (false, false) => Self::BC,
            (true, false) => Self::DE,
            (false, true) => Self::HL,
            (true, true) => Self::SP,
        }
    }
}

impl ArgumentWrite for ArgumentR16 {
    type Value = u16;

    fn set(&self, emulator: &mut Emulator, value: Self::Value) {
        match self {
            Self::BC => emulator.register_bc.set(value),
            Self::DE => emulator.register_de.set(value),
            Self::HL => emulator.register_hl.set(value),
            Self::SP => emulator.stack_pointer.0 = value,
        }
    }
}

impl ArgumentRead for ArgumentR16 {
    type Result = u16;

    /// Returns the value of the 16-bit register.
    fn get(&self, emulator: &Emulator) -> Self::Result {
        match self {
            Self::BC => emulator.register_bc.as_u16(),
            Self::DE => emulator.register_de.as_u16(),
            Self::HL => emulator.register_hl.as_u16(),
            Self::SP => emulator.stack_pointer.into(),
        }
    }
}

/// r16stk argument. Any of the general-purpose 16-bit registers and AF (accumulator and flags).
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ArgumentStkR16 {
    BC,
    DE,
    HL,
    AF,
}

impl ArgumentStkR16 {
    /// Used to parse the bits of the opcode to determine the register
    pub fn from_bits(b0: bool, b1: bool) -> Self {
        match (b0, b1) {
            (false, false) => Self::BC,
            (true, false) => Self::DE,
            (false, true) => Self::HL,
            (true, true) => Self::AF,
        }
    }
}

impl ArgumentWrite for ArgumentStkR16 {
    type Value = u16;

    fn set(&self, emulator: &mut Emulator, value: Self::Value) {
        match self {
            Self::BC => emulator.register_bc.set(value),
            Self::DE => emulator.register_de.set(value),
            Self::HL => emulator.register_hl.set(value),
            Self::AF => emulator.accumulator_and_flags.set(value),
        }
    }
}

impl ArgumentRead for ArgumentStkR16 {
    type Result = u16;

    /// Returns the value of the 16-bit register.
    fn get(&self, emulator: &Emulator) -> Self::Result {
        match self {
            Self::BC => emulator.register_bc.as_u16(),
            Self::DE => emulator.register_de.as_u16(),
            Self::HL => emulator.register_hl.as_u16(),
            Self::AF => emulator.accumulator_and_flags.as_u16(),
        }
    }
}

/// n8 argument. 8-bit integer constant.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ArgumentN8(pub u8);

impl ArgumentRead for ArgumentN8 {
    type Result = u8;

    /// Returns the value of the 8-bit integer constant.
    fn get(&self, _cpu: &Emulator) -> Self::Result {
        self.0
    }
}

/// n16 argument. 16-bit integer constant.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ArgumentN16(pub u16);

impl ArgumentRead for ArgumentN16 {
    type Result = u16;

    /// Returns the value of the 16-bit integer constant.
    fn get(&self, _cpu: &Emulator) -> Self::Result {
        self.0
    }
}

/// e8 argument. 8-bit offset (-128 to 127).
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ArgumentE8(pub i8);

impl ArgumentE8 {
    pub fn read(self) -> i8 {
        self.0
    }

    pub fn apply_offset(self, value: u16) -> u16 {
        if self.0 >= 0 {
            value.wrapping_add(self.0 as u16)
        } else {
            let offset = -(self.0 as i16);
            let offset = offset as u16;
            value.wrapping_sub(offset)
        }
    }

    pub fn apply_offset_with_flags(self, value: u16, flags: &mut u8) -> u16 {
        let unsigned_delta = self.as_u8() as u16;

        let carry = (value & 0xFF) + unsigned_delta > 0xFF;
        let half_carry = (value & 0x0F) + (unsigned_delta & 0x0F) > 0x0F;

        set_flag(flags, FLAG_HALF_CARRY, half_carry);
        set_flag(flags, FLAG_CARRY, carry);

        self.apply_offset(value)
    }

    /// Read value as u8 instead of i8
    pub fn as_u8(self) -> u8 {
        // Safety: i8 and u8 have the same size and alignment
        unsafe { std::mem::transmute(self.0) }
    }
}

impl ArgumentRead for ArgumentE8 {
    type Result = i8;

    /// Returns the value of the 8-bit offset.
    fn get(&self, _cpu: &Emulator) -> Self::Result {
        self.0
    }
}

/// u3 argument. 3-bit unsigned integer constant (0 to 7).
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ArgumentU3(U3);

impl ArgumentRead for ArgumentU3 {
    type Result = u8;

    /// Returns the value of the 3-bit unsigned integer constant.
    fn get(&self, _cpu: &Emulator) -> Self::Result {
        (*self).into()
    }
}

impl ArgumentU3 {
    pub fn from_bits(b0: bool, b1: bool, b2: bool) -> Self {
        Self(U3::from_bits([b0, b1, b2]))
    }
}

impl From<u8> for ArgumentU3 {
    fn from(value: u8) -> Self {
        ArgumentU3(value.into())
    }
}

impl From<ArgumentU3> for u8 {
    fn from(value: ArgumentU3) -> Self {
        value.0.into()
    }
}

/// cc argument. Condition code.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ArgumentCC {
    /// Z flag is reset
    NZ,
    /// Z flag is set
    Z,
    /// C flag is reset
    NC,
    /// C flag is set
    C,
}

impl ArgumentCC {
    pub fn from_bits(b0: bool, b1: bool) -> Self {
        match (b0, b1) {
            (false, false) => Self::NZ,
            (true, false) => Self::Z,
            (false, true) => Self::NC,
            (true, true) => Self::C,
        }
    }
}

impl ArgumentRead for ArgumentCC {
    type Result = bool;

    fn get(&self, emulator: &Emulator) -> Self::Result {
        let flags = emulator.accumulator_and_flags.low();

        match self {
            Self::NZ => !get_flag(flags, FLAG_ZERO),
            Self::Z => get_flag(flags, FLAG_ZERO),
            Self::NC => !get_flag(flags, FLAG_CARRY),
            Self::C => get_flag(flags, FLAG_CARRY),
        }
    }
}

pub trait ArgumentRead {
    type Result;

    fn get(&self, emulator: &Emulator) -> Self::Result;
}

impl<T: ArgumentRead<Result = u16>> MemoryAddress for T {
    fn get_at(&self, emulator: &Emulator) -> u8 {
        let address = self.get(emulator);
        emulator.get(address)
    }

    fn get_at_force(&self, emulator: &Emulator) -> u8 {
        let address = self.get(emulator);
        *emulator.get_force(address)
    }

    fn set_at(&self, emulator: &mut Emulator, value: u8) {
        let address = self.get(emulator);
        emulator.set(address, value);
    }

    fn set_at_force(&self, emulator: &mut Emulator, value: u8) {
        let address = self.get(emulator);
        emulator.set_force(address, value);
    }
}

pub trait ArgumentWrite {
    type Value;

    fn set(&self, emulator: &mut Emulator, value: Self::Value);
}

#[repr(u8)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ArgumentVec {
    Value0x00 = 0x00,
    Value0x08 = 0x08,
    Value0x10 = 0x10,
    Value0x18 = 0x18,
    Value0x20 = 0x20,
    Value0x28 = 0x28,
    Value0x30 = 0x30,
    Value0x38 = 0x38,
}

impl ArgumentVec {
    pub fn from_bits(b0: bool, b1: bool, b2: bool) -> Self {
        match (b0, b1, b2) {
            (false, false, false) => Self::Value0x00,
            (true, false, false) => Self::Value0x08,
            (false, true, false) => Self::Value0x10,
            (true, true, false) => Self::Value0x18,
            (false, false, true) => Self::Value0x20,
            (true, false, true) => Self::Value0x28,
            (false, true, true) => Self::Value0x30,
            (true, true, true) => Self::Value0x38,
        }
    }
}

impl ArgumentRead for ArgumentVec {
    type Result = u8;

    fn get(&self, _cpu: &Emulator) -> Self::Result {
        *self as u8
    }
}

#[test]
fn test_vec() {
    let emulator = Emulator::default();

    assert_eq!(ArgumentVec::Value0x00.get(&emulator), 0x00);
    assert_eq!(ArgumentVec::Value0x08.get(&emulator), 0x08);
    assert_eq!(ArgumentVec::Value0x10.get(&emulator), 0x10);
    assert_eq!(ArgumentVec::Value0x18.get(&emulator), 0x18);
    assert_eq!(ArgumentVec::Value0x20.get(&emulator), 0x20);
    assert_eq!(ArgumentVec::Value0x28.get(&emulator), 0x28);
    assert_eq!(ArgumentVec::Value0x30.get(&emulator), 0x30);
    assert_eq!(ArgumentVec::Value0x38.get(&emulator), 0x38);
}
