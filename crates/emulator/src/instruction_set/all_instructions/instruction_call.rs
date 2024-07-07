use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionCALL {
    /// Call address n16. This pushes the address of the instruction after the
    /// CALL on the stack, such that RET can pop it later; then, it executes
    /// an implicit JP n16.
    N16(ArgumentN16),
    /// Call address n16 if condition cc is met.
    CC_N16(ArgumentCC, ArgumentN16),
}

impl InstructionTrait for InstructionCALL {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match *self {
            Self::N16(address) => {
                exec_call(emulator, address);
                6
            }
            Self::CC_N16(condition, address) => {
                if condition.get(emulator) {
                    exec_call(emulator, address);
                    6
                } else {
                    3
                }
            }
        }
    }
}

fn exec_call(emulator: &mut Emulator, address: ArgumentN16) {
    let return_address = emulator.program_counter;
    emulator.program_counter = address.get(emulator).into();
    emulator.push_to_stack(return_address);
}
