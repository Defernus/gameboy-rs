use crate::*;

/// Push input value to the stack.
///
/// Flags: None affected.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionPUSH {
    AF,
    R16(ArgumentR16),
}

impl InstructionTrait for InstructionPUSH {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let input_value = match self {
            Self::AF => emulator.accumulator_and_flags.as_u16(),
            Self::R16(arg) => arg.get(emulator),
        };

        emulator.push_to_stack(input_value);

        4
    }
}
