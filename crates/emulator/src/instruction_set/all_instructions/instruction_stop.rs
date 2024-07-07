use crate::*;

/// Enter CPU very low power mode. Also used to switch between double and normal speed CPU modes in GBC.
///
/// Flags: None
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSTOP;

impl InstructionTrait for InstructionSTOP {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        emulator.is_in_low_power_mode = true;

        // TODO return proper value of cycles
        1
    }
}
