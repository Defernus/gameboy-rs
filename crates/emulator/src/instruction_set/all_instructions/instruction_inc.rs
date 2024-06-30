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
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        match self {
            Self::R8(reg) => {
                let result = reg.get(emulator).wrapping_add(1);

                let flags = emulator.accumulator_and_flags.low_mut();

                set_flag(flags, ZERO_FLAG, result == 0);
                set_flag(flags, SUBTRACT_FLAG, false);
                set_flag(flags, HALF_CARRY_FLAG, result > 0b0111);

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
