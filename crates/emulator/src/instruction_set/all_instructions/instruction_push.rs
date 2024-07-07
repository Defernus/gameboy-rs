use crate::*;

/// Push input value to the stack.
///
/// Flags: None affected.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionPUSH(pub ArgumentStkR16);

impl InstructionTrait for InstructionPUSH {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        let Self(reg) = *self;

        emulator.push_to_stack(reg.get(emulator));

        4
    }
}
