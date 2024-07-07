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
    fn execute(&self, emulator: &mut Emulator) -> usize {
        let mut value = emulator.accumulator_and_flags.high(); // B2 (178)

        let flags = emulator.accumulator_and_flags.low(); // 0111
        let half_carry = get_flag(flags, FLAG_HALF_CARRY); // 1
        let carry = get_flag(flags, FLAG_CARRY); // 1
        let subtract = get_flag(flags, FLAG_SUBTRACT); // 1

        let mut adjust = false;

        if subtract {
            // after a subtraction, only adjust if (half-)carry occurred
            if carry {
                value = value.wrapping_sub(0x60);
            }
            if half_carry {
                value = value.wrapping_sub(0x6);
            }
        } else {
            // after an addition, adjust if (half-)carry occurred or if result is out of bounds
            if carry || value > 0x99 {
                value = value.wrapping_add(0x60);
                adjust = true;
            }
            if half_carry || (value & 0x0f) > 0x09 {
                value = value.wrapping_add(0x6);
            }
        }

        let flags = emulator.accumulator_and_flags.low_mut();

        set_flag(flags, FLAG_ZERO, value == 0);
        set_flag(flags, FLAG_HALF_CARRY, false);
        if adjust {
            set_flag(flags, FLAG_CARRY, true);
        }

        emulator.accumulator_and_flags.set_high(value);

        1
    }
}
