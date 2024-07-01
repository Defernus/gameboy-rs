use crate::*;

/// Rotate register A left.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | 0 |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Set to rotated out bit |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRLCA;

impl InstructionTrait for InstructionRLCA {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        rotate_register_left(emulator, ArgumentR8::A);

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, FLAG_ZERO, false);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);

        1
    }
}
