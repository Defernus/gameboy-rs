use crate::*;

/// Swap the upper 4 bits in r8 and the lower 4 ones.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | 0 |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSWAP(pub ArgumentR8);

impl InstructionTrait for InstructionSWAP {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(reg) = *self;

        let value = reg.get(emulator);

        let result = swap_halves(value);

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, FLAG_ZERO, result == 0);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);
        set_flag(flags, FLAG_CARRY, false);

        reg.set(emulator, result);

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}

#[inline(always)]
fn swap_halves(value: u8) -> u8 {
    (value << 4) | (value >> 4)
}

#[test]
fn test_swap_halves() {
    assert_eq!(swap_halves(0x00), 0x00);
    assert_eq!(swap_halves(0x02), 0x20);
    assert_eq!(swap_halves(0x20), 0x02);
    assert_eq!(swap_halves(0x4a), 0xa4);
    assert_eq!(swap_halves(0xff), 0xff);
    assert_eq!(swap_halves(0x33), 0x33);
    assert_eq!(swap_halves(0x01), 0x10);
}
