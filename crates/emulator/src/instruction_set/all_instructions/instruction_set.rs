use crate::*;

/// Set bit u3 in r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSET(pub ArgumentU3, pub ArgumentR8);

impl InstructionTrait for InstructionSET {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(bit, reg) = *self;

        let bit = bit.get(emulator);

        let value = reg.get(emulator);
        reg.set(emulator, value | (1 << bit));

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}
