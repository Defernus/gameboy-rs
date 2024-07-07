use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionJP {
    /// Jump to address n16; effectively, store n16 into PC
    N16(ArgumentN16),
    /// Jump to address n16 if condition cc is met.
    CC_N16(ArgumentCC, ArgumentN16),
    /// Jump to address in HL; effectively, load PC with value in register HL.
    HL,
}

impl InstructionTrait for InstructionJP {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match self {
            Self::N16(to_address) => {
                emulator.program_counter = to_address.get(emulator).into();
                4
            }
            Self::CC_N16(condition, to_address) => {
                if condition.get(emulator) {
                    emulator.program_counter = to_address.get(emulator).into();
                    4
                } else {
                    3
                }
            }
            Self::HL => {
                emulator.program_counter = emulator.register_hl.as_u16().into();
                1
            }
        }
    }
}
