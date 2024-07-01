use crate::*;

/// Rotate r8 right, through the carry flag.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Set according to result. |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRR(pub ArgumentR8);

impl InstructionTrait for InstructionRR {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(reg) = *self;

        let result = rotate_register_right_carry(emulator, reg);

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, FLAG_ZERO, result == 0);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}

/// Rotate value in register right, through the carry flag.
pub fn rotate_register_right_carry(emulator: &mut Emulator, reg: ArgumentR8) -> u8 {
    let value = reg.get(emulator);

    let flags = emulator.accumulator_and_flags.low();
    let old_carry = get_flag(flags, FLAG_CARRY) as u8;

    let new_carry = value & 0b0000_0001 != 0;

    let flags = emulator.accumulator_and_flags.low_mut();
    set_flag(flags, FLAG_CARRY, new_carry);

    let result = (value >> 1) | (old_carry << 7);
    reg.set(emulator, result);

    result
}
