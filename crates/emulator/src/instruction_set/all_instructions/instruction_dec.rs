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
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        match self {
            Self::R8(reg) => {
                let prev_4th_bit = reg.get(emulator) & 0b0001_0000 != 0;

                let result = reg.get(emulator).wrapping_sub(1);

                let new_4th_bit = reg.get(emulator) & 0b0001_0000 != 0;

                let flags = emulator.accumulator_and_flags.low_mut();

                set_flag(flags, ZERO_FLAG, result == 0);
                set_flag(flags, SUBTRACT_FLAG, true);
                set_flag(flags, HALF_CARRY_FLAG, prev_4th_bit && !new_4th_bit);

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
