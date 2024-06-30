use crate::*;

/// Store into A the bitwise OR of input value and A.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | 0 |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionOR {
    A_R8(ArgumentR8),
    A_N8(ArgumentN8),
}

impl InstructionTrait for InstructionOR {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let (input_value, cycles) = match self {
            Self::A_R8(arg @ ArgumentR8::AtHL) => (arg.get(emulator), 2),
            Self::A_R8(arg) => (arg.get(emulator), 1),
            Self::A_N8(arg) => (arg.get(emulator), 2),
        };

        let register_a = emulator.accumulator_and_flags.low_mut();

        let result = *register_a | input_value;
        *register_a = result;

        set_flag(register_a, ZERO_FLAG, result == 0);
        set_flag(register_a, SUBTRACT_FLAG, false);
        set_flag(register_a, HALF_CARRY_FLAG, false);
        set_flag(register_a, CARRY_FLAG, false);

        cycles
    }
}
