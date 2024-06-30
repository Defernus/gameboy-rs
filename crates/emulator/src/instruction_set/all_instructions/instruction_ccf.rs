use crate::*;

/// Complement Carry Flag
///
/// Invert the carry flag.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Inverted |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionCCF;

impl InstructionTrait for InstructionCCF {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, false);
        invert_flag(flags, CARRY_FLAG);

        1
    }
}
