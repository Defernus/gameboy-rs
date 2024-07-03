use crate::StackValue;

pub const INITIAL_PROGRAM_COUNTER: u16 = 0x100;

/// The program counter register. Initialized to `0x100`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ProgramCounter(pub u16);

impl ProgramCounter {
    pub fn advance(&mut self, value: u16) {
        self.0 = self.0.wrapping_add(value);
    }

    pub fn post_increment(&mut self, value: u16) -> u16 {
        let result = self.0;
        self.advance(value);
        result
    }
}

impl Default for ProgramCounter {
    fn default() -> Self {
        Self(INITIAL_PROGRAM_COUNTER)
    }
}

impl From<ProgramCounter> for u16 {
    fn from(pc: ProgramCounter) -> u16 {
        pc.0
    }
}

impl From<u16> for ProgramCounter {
    fn from(value: u16) -> ProgramCounter {
        ProgramCounter(value)
    }
}

impl StackValue for ProgramCounter {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(u16::from_bytes(bytes))
    }

    fn to_bytes(self) -> Vec<u8> {
        self.0.to_bytes()
    }
}
