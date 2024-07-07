use crate::*;

/// Shift Right Arithmetically register r8 (bit 7 of r8 is unchanged).
///
/// `[7 -> ... -> 0] -> C`
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Set to the bit that was shifted out. |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSRA(pub ArgumentR8);

impl InstructionTrait for InstructionSRA {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        let Self(reg) = *self;

        let value = reg.get(emulator);

        let carry = value & 0b1 != 0;
        let result = value & 0b1000_0000 | value >> 1;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, FLAG_ZERO, result == 0);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);
        set_flag(flags, FLAG_CARRY, carry);

        reg.set(emulator, result);

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}
