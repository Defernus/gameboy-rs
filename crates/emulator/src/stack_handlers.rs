use crate::*;

impl Emulator {
    /// Push value to stack at current SP and decrement SP by size of value.
    /// (e.g. `2` for `u16`, `1` for `u8`)
    pub fn push_to_stack<T: StackValue>(&mut self, value: T) {
        for byte in value.to_bytes().into_iter().rev() {
            self.stack_pointer.decrement();
            self.memory.set(self.stack_pointer.into(), byte);
        }
    }

    /// Pop value from stack at current SP and increment SP by size of value.
    /// (e.g. `2` for `u16`, `1` for `u8`)
    pub fn pop_from_stack<T: StackValue>(&mut self) -> T {
        let size = std::mem::size_of::<T>();

        let mut bytes = Vec::with_capacity(size);
        for _ in 0..size {
            bytes.push(self.memory.get(self.stack_pointer.into()));
            self.stack_pointer.increment();
        }

        T::from_bytes(&bytes)
    }
}

pub trait StackValue: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(self) -> Vec<u8>;
}

impl StackValue for u8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0]
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self]
    }
}

impl StackValue for u16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    fn to_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
