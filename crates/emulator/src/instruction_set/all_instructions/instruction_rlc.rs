use crate::*;

/// Rotate r8 left.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Set to rotated out bit |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRLC(pub ArgumentR8);

impl InstructionTrait for InstructionRLC {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(reg) = *self;

        let result = rotate_register_left_carry(emulator, reg);

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, ZERO_FLAG, result == 0);
        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, false);

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}

/// Rotate register left. Set carry flag to the value of the bit that was rotated out.
pub fn rotate_register_left(emulator: &mut Emulator, reg: ArgumentR8) -> u8 {
    let value = reg.get(emulator);

    let carry = value & 0b1000_0000 != 0;
    let flags = emulator.accumulator_and_flags.low_mut();
    set_flag(flags, CARRY_FLAG, carry);

    let result = (value << 1) | carry as u8;
    reg.set(emulator, result);

    result
}
