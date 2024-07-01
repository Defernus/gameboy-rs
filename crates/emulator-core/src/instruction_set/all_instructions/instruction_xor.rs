use crate::*;

/// Bitwise XOR between input value and A.
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
pub enum InstructionXOR {
    A_R8(ArgumentR8),
    A_N8(ArgumentN8),
}

impl InstructionTrait for InstructionXOR {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let (input_value, cycles) = match self {
            Self::A_R8(arg @ ArgumentR8::AtHL) => (arg.get(emulator), 2),
            Self::A_R8(arg) => (arg.get(emulator), 1),
            Self::A_N8(arg) => (arg.get(emulator), 2),
        };

        let register_a = emulator.accumulator_and_flags.high();

        let result = register_a ^ input_value;

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, FLAG_ZERO, result == 0);
        set_flag(flags, FLAG_SUBTRACT, false);
        set_flag(flags, FLAG_HALF_CARRY, false);
        set_flag(flags, FLAG_CARRY, false);

        emulator.accumulator_and_flags.set_high(result);

        cycles
    }
}

#[test]
fn test_xor_flags() {
    let mut emulator = Emulator::default();

    // reset flags
    emulator.accumulator_and_flags.set_low(0x00);
    // reset A register
    emulator.accumulator_and_flags.set_high(0x00);

    InstructionXOR::A_R8(ArgumentR8::A).execute(&mut emulator);

    // 0x00 ^ 0x00 = 0x00
    assert_eq!(emulator.accumulator_and_flags.high(), 0x00);

    // Z flag should be set, all other flags should be reset
    assert_eq!(emulator.accumulator_and_flags.low(), 0b1000_0000);
}
