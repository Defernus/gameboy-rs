use crate::*;

/// Rotate register A right, through the carry flag.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | 0 |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Set according to result. |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRRA;

impl InstructionTrait for InstructionRRA {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        rotate_register_right_carry(emulator, ArgumentR8::A);

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, ZERO_FLAG, false);
        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, false);

        1
    }
}
