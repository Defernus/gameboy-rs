use crate::*;

/// Bitwise AND between input value and A.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 1 |
/// | C    | 0 |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionAND {
    A_R8(ArgumentR8),
    A_N8(ArgumentN8),
}

impl InstructionTrait for InstructionAND {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let (value, cycles) = match self {
            InstructionAND::A_R8(arg @ ArgumentR8::AtHL) => (arg.get(emulator), 3),
            InstructionAND::A_R8(arg) => (arg.get(emulator), 2),
            InstructionAND::A_N8(arg) => (arg.get(emulator), 3),
        };

        let register_a = emulator.accumulator_and_flags.high();

        let result = register_a & value;
        emulator.accumulator_and_flags.set_high(result);

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, ZERO_FLAG, result == 0);
        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, true);
        set_flag(flags, CARRY_FLAG, false);

        cycles
    }
}
