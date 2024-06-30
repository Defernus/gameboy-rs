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
            (false, false, true) => Self::C,
            (false, true, false) => Self::D,
            (false, true, true) => Self::E,
            (true, false, false) => Self::H,
            (true, false, true) => Self::L,
            (true, true, false) => Self::AtHL,
            (true, true, true) => Self::A,
        }
    }
}

impl ArgumentWrite for ArgumentR8 {
    type Result = u8;

    /// Returns a mutable reference to the 8-bit register or memory at HL.
    fn get_mut<'a>(&self, emulator: &'a mut Emulator) -> &'a mut Self::Result {
        match self {
            Self::B => emulator.register_bc.high_mut(),
            Self::C => emulator.register_bc.low_mut(),
            Self::D => emulator.register_de.high_mut(),
            Self::E => emulator.register_de.low_mut(),
            Self::H => emulator.register_hl.high_mut(),
            Self::L => emulator.register_hl.low_mut(),
            Self::AtHL => emulator.memory.get_mut(emulator.register_hl.as_u16()),
            Self::A => emulator.accumulator_and_flags.high_mut(),
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
            Self::AtHL => emulator.memory.get(emulator.register_hl.as_u16()),
            Self::A => emulator.accumulator_and_flags.high(),
        }
    }
}

/// r16 argument. Any of the general-purpose 16-bit registers
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
            (false, true) => Self::DE,
            (true, false) => Self::HL,
            (true, true) => Self::SP,
        }
    }
}

impl ArgumentWrite for ArgumentR16 {
    type Result = u16;

    /// Returns a mutable reference to the 16-bit register.
    fn get_mut<'a>(&self, emulator: &'a mut Emulator) -> &'a mut Self::Result {
        match self {
            Self::BC => emulator.register_bc.as_u16_mut(),
            Self::DE => emulator.register_de.as_u16_mut(),
            Self::HL => emulator.register_hl.as_u16_mut(),
            Self::SP => &mut emulator.stack_pointer.0,
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

impl MemoryAccess for ArgumentR16 {
    /// Returns the value of the memory at the address in the 16-bit register.
    fn at(&self, emulator: &Emulator) -> u8 {
        emulator.memory.get(self.get(emulator))
    }

    /// Returns a mutable reference to the memory at the address in the 16-bit register.
    fn at_mut<'a>(&self, emulator: &'a mut Emulator) -> &'a mut u8 {
        emulator.memory.get_mut(self.get(emulator))
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

impl MemoryAccess for ArgumentN16 {
    /// Returns the value of the memory at the address in the 16-bit integer constant.
    fn at(&self, emulator: &Emulator) -> u8 {
        emulator.memory.get(self.get(emulator))
    }

    /// Returns a mutable reference to the memory at the address in the 16-bit integer constant.
    fn at_mut<'a>(&self, emulator: &'a mut Emulator) -> &'a mut u8 {
        emulator.memory.get_mut(self.get(emulator))
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
            value.wrapping_sub((-self.0) as u16)
        }
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
pub struct ArgumentU3(u8);

impl ArgumentRead for ArgumentU3 {
    type Result = u8;

    /// Returns the value of the 3-bit unsigned integer constant.
    fn get(&self, _cpu: &Emulator) -> Self::Result {
        (*self).into()
    }
}

impl ArgumentU3 {
    pub fn from_bits(b0: bool, b1: bool, b2: bool) -> Self {
        let value = (b0 as u8) << 2 | (b1 as u8) << 1 | b2 as u8;
        Self(value)
    }
}

impl From<u8> for ArgumentU3 {
    fn from(value: u8) -> Self {
        ArgumentU3(value & 0b111)
    }
}

impl From<ArgumentU3> for u8 {
    fn from(value: ArgumentU3) -> Self {
        value.0 & 0b111
    }
}

/// cc argument. Condition code.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ArgumentCC(pub u8); // TODO implement condition code argument

impl ArgumentCC {
    pub fn read(self, emulator: &Emulator) -> bool {
        (self.0 & emulator.accumulator_and_flags.low()) & FLAGS_MASK != 0
    }

    pub fn from_bits(b0: bool, b1: bool) -> Self {
        let value = (b0 as u8) << 1 | b1 as u8;
        Self(value)
    }
}

pub trait ArgumentRead {
    type Result;

    fn get(&self, emulator: &Emulator) -> Self::Result;
}

pub trait ArgumentWrite {
    type Result;

    fn get_mut<'a>(&self, emulator: &'a mut Emulator) -> &'a mut Self::Result;

    fn set(&self, emulator: &mut Emulator, value: Self::Result) {
        *self.get_mut(emulator) = value;
    }
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
            (false, false, true) => Self::Value0x08,
            (false, true, false) => Self::Value0x10,
            (false, true, true) => Self::Value0x18,
            (true, false, false) => Self::Value0x20,
            (true, false, true) => Self::Value0x28,
            (true, true, false) => Self::Value0x30,
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
