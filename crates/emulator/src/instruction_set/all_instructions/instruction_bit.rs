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
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let (bit, register) = (self.0, self.1);

        let value = register.get(emulator);
        let bit = bit.get(emulator);

        let bit_set = (value & (1 << bit)) != 0;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, ZERO_FLAG, !bit_set);
        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, true);

        if register == ArgumentR8::AtHL {
            3
        } else {
            2
        }
    }
}
