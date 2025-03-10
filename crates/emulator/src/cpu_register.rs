use crate::*;

const HIGH_REGISTER: usize = 1;
const LOW_REGISTER: usize = 0;

/// A 16-bit register.
///
/// Divided into two 8-bit registers: high and low.
///
/// ```
/// 0x12_34
///   |  |
///   |  +-- low
///   +----- high
/// ```
///
/// For example the `BC` register is divided into `B` - high and `C` - low.
#[derive(Default, Debug, Clone, Copy)]
pub struct CpuRegister([u8; 2]);

impl CpuRegister {
    #[inline(always)]
    pub fn new() -> Self {
        Self([0; 2])
    }

    #[inline(always)]
    pub fn increment(&mut self) {
        let value = self.as_u16_mut();
        *value = value.wrapping_add(1);
    }

    #[inline(always)]
    pub fn decrement(&mut self) {
        let value = self.as_u16_mut();
        *value = value.wrapping_sub(1);
    }

    #[inline(always)]
    pub fn low(self) -> u8 {
        self.0[LOW_REGISTER]
    }

    #[inline(always)]
    pub fn set_low(&mut self, value: u8) {
        self.0[LOW_REGISTER] = value;
    }

    #[inline(always)]
    pub fn high(self) -> u8 {
        self.0[HIGH_REGISTER]
    }

    #[inline(always)]
    pub fn set_high(&mut self, value: u8) {
        self.0[HIGH_REGISTER] = value;
    }

    #[inline(always)]
    pub fn flag(self, flag: u8) -> bool {
        self.low() & flag == flag
    }

    #[inline(always)]
    pub fn as_u16(self) -> u16 {
        u16::from_le_bytes(self.0)
    }

    #[inline(always)]
    pub fn as_u16_mut(&mut self) -> &mut u16 {
        // Safety: `u16` has the same memory layout as `[u8; 2]`
        unsafe { std::mem::transmute(&mut self.0) }
    }

    #[inline(always)]
    pub fn set(&mut self, value: u16) {
        *self.as_u16_mut() = value;
    }

    #[inline(always)]
    pub fn high_mut(&mut self) -> &mut u8 {
        &mut self.0[HIGH_REGISTER]
    }

    #[inline(always)]
    pub fn low_mut(&mut self) -> &mut u8 {
        &mut self.0[LOW_REGISTER]
    }
}
impl MemoryAddress for CpuRegister {
    fn get_at(&self, emulator: &Emulator) -> u8 {
        emulator.get(self.as_u16())
    }

    fn get_at_force(&self, emulator: &Emulator) -> u8 {
        *emulator.get_force(self.as_u16())
    }

    fn set_at(&self, emulator: &mut Emulator, value: u8) {
        emulator.set(self.as_u16(), value);
    }

    fn set_at_force(&self, emulator: &mut Emulator, value: u8) {
        emulator.set_force(self.as_u16(), value);
    }
}
