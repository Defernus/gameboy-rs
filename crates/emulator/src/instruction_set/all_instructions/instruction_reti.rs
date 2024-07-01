use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRETI;

impl InstructionTrait for InstructionRETI {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        emulator.delayed_ime_set = true;
        emulator.program_counter = emulator.pop_from_stack();

        4
    }
}
