use crate::*;

/// Rotate register A left, through the carry flag.
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
pub struct InstructionRLA;

impl InstructionTrait for InstructionRLA {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        rotate_register_left_carry(emulator, ArgumentR8::A);

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, FLAG_ZERO, false);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);

        1
    }
}
