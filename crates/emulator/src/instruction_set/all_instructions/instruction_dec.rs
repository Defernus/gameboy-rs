use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionDEC {
    /// Decrement value in register r8 by 1.
    ///
    /// Flags:
    ///
    /// | Flag | Value |
    /// |------|-------|
    /// | Z    | Set if result is 0. |
    /// | N    | 1 |
    /// | H    | Set if borrow from bit 4. |
    R8(ArgumentR8),
    /// Decrement value in register r16 by 1.
    ///
    /// Flags: None affected.
    R16(ArgumentR16),
}

impl InstructionTrait for InstructionDEC {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match self {
            Self::R8(reg) => {
                let prev_value = reg.get(emulator);
                let result = prev_value.wrapping_sub(1);

                let half_carry = (prev_value & 0x0F) == 0;

                let flags = emulator.accumulator_and_flags.low_mut();

                set_flag(flags, FLAG_ZERO, result == 0);
                set_flag(flags, FLAG_SUBTRACT, true);
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
                let result = value.wrapping_sub(1);

                reg.set(emulator, result);

                2
            }
        }
    }
}
