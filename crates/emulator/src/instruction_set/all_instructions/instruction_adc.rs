use crate::*;

/// Add input value plus the carry flag to A.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | Set if carry from bit 3. |
/// | C    | Set if carry from bit 7. |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionADC {
    A_R8(ArgumentR8),
    A_N8(ArgumentN8),
}

impl InstructionTrait for InstructionADC {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let (input_value, cycles) = match self {
            InstructionADC::A_R8(arg @ ArgumentR8::AtHL) => (arg.get(emulator), 2),
            InstructionADC::A_R8(arg) => (arg.get(emulator), 1),
            InstructionADC::A_N8(arg) => (arg.get(emulator), 2),
        };

        let register_a = emulator.accumulator_and_flags.low();

        let flags = emulator.accumulator_and_flags.low();
        let carry = get_flag(flags, CARRY_FLAG) as u8;

        let result = register_a.wrapping_add(input_value).wrapping_add(carry);

        let half_carry = (register_a & 0xFF) + (input_value & 0xFF) + carry > 0xF;
        let carry = (register_a as u16) + (input_value as u16) + (carry as u16) > 0xFF;

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, ZERO_FLAG, result == 0);
        invert_flag(flags, SUBTRACT_FLAG);
        set_flag(flags, HALF_CARRY_FLAG, half_carry);
        set_flag(flags, CARRY_FLAG, carry);

        emulator.accumulator_and_flags.set_low(result);

        cycles
    }
}
