pub const INITIAL_STACK_POINTER: u16 = 0xFFFE;

/// Stack pointer register. Initialized to `0xFFFE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct StackPointer(pub u16);

impl StackPointer {
    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.0 = self.0.wrapping_sub(1);
    }
}

impl Default for StackPointer {
    fn default() -> Self {
        Self(INITIAL_STACK_POINTER)
    }
}

impl From<StackPointer> for u16 {
    fn from(pc: StackPointer) -> u16 {
        pc.0
    }
}

impl From<u16> for StackPointer {
    fn from(value: u16) -> StackPointer {
        StackPointer(value)
    }
}
