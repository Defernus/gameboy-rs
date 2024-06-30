use crate::*;

/// Return from subroutine. This is basically a POP PC (if such an instruction
/// existed).
///
/// If a condition is specified, the instruction will only be executed if the
/// condition is met.
///
/// Flags: None affected.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionRET(pub Option<ArgumentCC>);

impl InstructionTrait for InstructionRET {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let mut cycles = 4;

        if let Some(cc) = self.0 {
            if !cc.read(emulator) {
                return 2;
            }

            cycles += 1
        }

        emulator.program_counter = emulator.pop_from_stack();

        cycles
    }
}
