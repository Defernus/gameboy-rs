use crate::*;

/// Enable Interrupts by setting the IME flag. The flag is only set after the instruction following EI.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionEI;

impl InstructionTrait for InstructionEI {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        emulator.delayed_ime_set = true;

        1
    }
}
