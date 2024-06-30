use crate::*;

/// Shift Right Logically register r8.
///
/// `0 -> [7 -> ... -> 0] -> C`
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSRL(pub ArgumentR8);

impl InstructionTrait for InstructionSRL {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(reg) = *self;

        let value = reg.get(emulator);

        let carry = value & 0b1 != 0;
        let result = value >> 1;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, ZERO_FLAG, result == 0);
        set_flag(flags, SUBTRACT_FLAG, false);
        set_flag(flags, HALF_CARRY_FLAG, false);
        set_flag(flags, CARRY_FLAG, carry);

        reg.set(emulator, result);

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}
