use crate::*;

/// Disable Interrupts by clearing the IME flag.
///
/// Flags: None affected.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionDI;

impl InstructionTrait for InstructionDI {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        emulator.ime_flag = false;

        1
    }
}
