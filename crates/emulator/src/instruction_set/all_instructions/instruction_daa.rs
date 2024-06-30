use crate::*;

/// Decimal Adjust Accumulator to get a correct BCD representation after an arithmetic instruction.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | H    | 0 |
/// | C    | Set or reset depending on the operation. |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InstructionDAA;

impl InstructionTrait for InstructionDAA {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        let mut register_a = emulator.accumulator_and_flags.high();

        let flags = emulator.accumulator_and_flags.low();
        let half_carry = get_flag(flags, HALF_CARRY_FLAG);
        let carry = get_flag(flags, CARRY_FLAG);

        if register_a & 0x0F > 9 || half_carry {
            register_a = register_a.wrapping_add(0x06);
        }

        let mut adjust = false;

        if register_a & 0xF0 > 0x90 || carry {
            adjust = true;

            register_a = register_a.wrapping_add(0x60);
        };

        let flags = emulator.accumulator_and_flags.low_mut();
        set_flag(flags, ZERO_FLAG, register_a == 0);
        set_flag(flags, HALF_CARRY_FLAG, false);
        set_flag(flags, CARRY_FLAG, adjust);

        emulator.accumulator_and_flags.set_high(register_a);

        1
    }
}
