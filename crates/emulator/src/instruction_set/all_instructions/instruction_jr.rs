use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionJR {
    E8(ArgumentE8),
    CC_E8(ArgumentCC, ArgumentE8),
}

impl InstructionTrait for InstructionJR {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match self {
            Self::E8(offset) => {
                emulator.program_counter =
                    offset.apply_offset(emulator.program_counter.into()).into();
                3
            }
            Self::CC_E8(condition, offset) => {
                if condition.get(emulator) {
                    emulator.program_counter =
                        offset.apply_offset(emulator.program_counter.into()).into();
                    3
                } else {
                    2
                }
            }
        }
    }
}
