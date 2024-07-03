use crate::*;

/// Pop value from stack and store it in 16-bit register.
///
/// If af register is specified, flags will be set to the value popped from
/// the stack (because flags are stored in the lower byte of af register).
///
/// If any other register is specified, flags will not be affected.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionPOP(pub ArgumentStkR16);

impl InstructionTrait for InstructionPOP {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(reg) = *self;

        let value = emulator.pop_from_stack();

        reg.set(emulator, value);
        if reg == ArgumentStkR16::AF {
            *emulator.accumulator_and_flags.low_mut() &= FLAGS_MASK;
        }

        3
    }
}
