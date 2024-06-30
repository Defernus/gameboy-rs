use crate::*;

/// ComPLement accumulator (A = ~A).
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | N    | 1     |
/// | H    | 1     |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionCPL;

impl InstructionTrait for InstructionCPL {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let register_a = emulator.accumulator_and_flags.high_mut();
        *register_a ^= 0xFF;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, ZERO_FLAG, true);
        set_flag(flags, SUBTRACT_FLAG, true);

        1
    }
}
