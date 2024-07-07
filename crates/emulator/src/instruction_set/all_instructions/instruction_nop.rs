use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionNOP;

impl InstructionTrait for InstructionNOP {
    fn execute(&self, _cpu: &mut Emulator) -> usize {
        1
    }
}
