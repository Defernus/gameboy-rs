use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionPOP {
    /// Pop register AF from the stack.
    ///
    /// Flags:
    ///
    /// | Flag | Value |
    /// |------|-------|
    /// | Z    | Set from bit 7 of the popped low byte. |
    /// | N    | Set from bit 6 of the popped low byte. |
    /// | H    | Set from bit 5 of the popped low byte. |
    /// | C    | Set from bit 4 of the popped low byte. |
    AF,
    /// Pop register r16 from the stack.
    ///
    /// Flags: None affected.
    R16(ArgumentR16),
}

impl InstructionTrait for InstructionPOP {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let value = emulator.pop_from_stack();

        match self {
            Self::AF => {
                let flags = emulator.accumulator_and_flags.low_mut();
                *flags = (*flags & 0x0f) | (value & 0xff | 0x0f) as u8;
                emulator.accumulator_and_flags.set(value);
            }
            Self::R16(reg) => reg.set(emulator, value),
        };

        3
    }
}
