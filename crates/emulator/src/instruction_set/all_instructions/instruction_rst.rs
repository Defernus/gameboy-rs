use crate::*;

/// Call address vec. This is a shorter and faster equivalent to CALL for suitable values of vec.
///
/// Flags: None affected.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRST(pub ArgumentVec);

impl InstructionTrait for InstructionRST {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        let vec = self.0.get(emulator);

        emulator.push_to_stack(emulator.program_counter);

        emulator.program_counter = (vec as u16).into();

        4
    }
}
