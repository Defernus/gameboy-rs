use crate::*;

/// Test bit u3 in register r8, set the zero flag if bit not set.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if the selected bit is 0. |
/// | N    | 0 |
/// | H    | 1 |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionBIT(pub ArgumentU3, pub ArgumentR8);

impl InstructionTrait for InstructionBIT {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        let (bit, register) = (self.0, self.1);

        let value = register.get(emulator);
        let bit = bit.get(emulator);

        let bit_set = (value & (1 << bit)) != 0;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, FLAG_ZERO, !bit_set);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, true);

        if register == ArgumentR8::AtHL {
            3
        } else {
            2
        }
    }
}
