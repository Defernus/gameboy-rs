use crate::*;

/// Shift Left Arithmetically r8.
///
/// `C <- [7 <- ... <- 0] <- 0`
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 0 |
/// | H    | 0 |
/// | C    | Set to the bit that was shifted out. |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionSLA(pub ArgumentR8);

impl InstructionTrait for InstructionSLA {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let Self(reg) = *self;

        let value = reg.get(emulator);

        let carry = value & 0b1000_0000 != 0;
        let result = value << 1;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, FLAG_ZERO, result == 0);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);
        set_flag(flags, FLAG_CARRY, carry);

        reg.set(emulator, result);

        if reg == ArgumentR8::AtHL {
            4
        } else {
            2
        }
    }
}
