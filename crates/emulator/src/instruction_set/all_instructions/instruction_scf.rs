use crate::*;

/// Set Carry Flag.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | N    | 0 |
/// | H    | 0 |
/// | C    | 1 |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSCF;

impl InstructionTrait for InstructionSCF {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, false);
        set_flag(flags, CARRY_FLAG, true);

        1
    }
}
