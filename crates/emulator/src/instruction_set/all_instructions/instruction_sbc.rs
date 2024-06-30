use crate::*;

/// Subtract the input value and the carry flag from A.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 1 |
/// | H    | Set if borrow from bit 4. |
/// | C    | Set if borrow (e.g. if (r8 + carry) > A). |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionSBC {
    A_R8(ArgumentR8),
    A_N8(ArgumentN8),
}

impl InstructionTrait for InstructionSBC {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let (input_value, cycles) = match self {
            Self::A_R8(arg @ ArgumentR8::AtHL) => (arg.get(emulator), 2),
            Self::A_R8(arg) => (arg.get(emulator), 1),
            Self::A_N8(arg) => (arg.get(emulator), 2),
        };

        let register_a = emulator.accumulator_and_flags.low();

        let flags = emulator.accumulator_and_flags.low();
        let carry = get_flag(flags, CARRY_FLAG) as u8;

        let result = register_a.wrapping_sub(input_value).wrapping_sub(carry);

        let half_carry = (register_a & 0xF) < (input_value & 0xF) + carry;
        let carry = (register_a as u16) < (input_value as u16) + (carry as u16);

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, ZERO_FLAG, result == 0);
        set_flag(flags, SUBTRACT_FLAG, true);
        set_flag(flags, HALF_CARRY_FLAG, half_carry);
        set_flag(flags, CARRY_FLAG, carry);

        emulator.accumulator_and_flags.set_low(result);

        cycles
    }
}
