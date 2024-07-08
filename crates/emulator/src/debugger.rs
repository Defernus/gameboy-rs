use crate::{Emulator, Instruction};

pub trait EmulatorDebugger {
    fn on_after_instruction(&mut self, emulator: &Emulator, opcode: u8, instruction: Instruction);
}
