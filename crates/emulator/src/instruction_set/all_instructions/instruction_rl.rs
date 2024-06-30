use crate::*;

/// Rotate bits in register r8 left, through the carry flag.
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
pub struct InstructionRL(pub ArgumentR8);

impl InstructionTrait for InstructionRL {
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

/// Rotate value in register left, through the carry flag.
pub fn rotate_register_left_carry(emulator: &mut Emulator, reg: ArgumentR8) -> u8 {
    let value = reg.get(emulator);

    let flags = emulator.accumulator_and_flags.low();
    let old_carry = get_flag(flags, CARRY_FLAG) as u8;

    let new_carry = value & 0b1000_0000 != 0;
    let flags = emulator.accumulator_and_flags.low_mut();
    set_flag(flags, CARRY_FLAG, new_carry);

    let result = (value << 1) | old_carry;
    reg.set(emulator, result);

    result
}
