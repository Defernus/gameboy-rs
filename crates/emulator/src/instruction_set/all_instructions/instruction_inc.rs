use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionINC {
    /// Increment value in register r8 by 1.
    ///
    /// Flags:
    ///
    /// | flag | value |
    /// |------|-------|
    /// | Z | Set if result is 0. |
    /// | N | 0 |
    /// | H | Set if overflow from bit 3. |
    R8(ArgumentR8),
    /// Increment value in register r16 by 1.
    ///
    /// Flags: None affected.
    R16(ArgumentR16),
}

impl InstructionTrait for InstructionINC {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match self {
            Self::R8(reg) => {
                let prev_value = reg.get(emulator);
                let result = prev_value.wrapping_add(1);

                let flags = emulator.accumulator_and_flags.low_mut();

                let half_carry = (prev_value & 0x0F) + 1 > 0x0F;

                set_flag(flags, FLAG_ZERO, result == 0);
                set_flag(flags, FLAG_SUBTRACT, false);
                set_flag(flags, FLAG_HALF_CARRY, half_carry);

                reg.set(emulator, result);

                if *reg == ArgumentR8::AtHL {
                    3
                } else {
                    1
                }
            }
            Self::R16(reg) => {
                let value = reg.get(emulator);
                let result = value.wrapping_add(1);

                reg.set(emulator, result);

                2
            }
        }
    }
}
