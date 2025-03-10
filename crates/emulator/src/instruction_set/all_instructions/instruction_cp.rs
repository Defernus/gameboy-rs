use crate::*;

/// Subtract the input value from A and set flags accordingly, but don't store
/// the result. This is useful for ComParing values.
///
/// Flags:
///
/// | Flag | Value |
/// |------|-------|
/// | Z    | Set if result is 0. |
/// | N    | 1 |
/// | H    | Set if borrow from bit 4. |
/// | C    | Set if borrow (e.g. if r8 > A). |
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionCP {
    A_R8(ArgumentR8),
    A_N8(ArgumentN8),
}

impl InstructionTrait for InstructionCP {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match self {
            Self::A_R8(reg) => {
                let register_a = emulator.accumulator_and_flags.high();
                let value = reg.get(emulator);

                update_flags(emulator, register_a, value);

                if *reg == ArgumentR8::AtHL {
                    2
                } else {
                    1
                }
            }
            Self::A_N8(arg) => {
                let register_a = emulator.accumulator_and_flags.high();
                let value = arg.get(emulator);

                update_flags(emulator, register_a, value);

                2
            }
        }
    }
}

fn update_flags(emulator: &mut Emulator, register_a: u8, value: u8) {
    let borrow = register_a.wrapping_sub(value);

    let half_carry = (register_a & 0x0F) < (value & 0x0F);
    let carry = register_a < value;

    let flags = emulator.accumulator_and_flags.low_mut();

    set_flag(flags, FLAG_ZERO, borrow == 0);
    set_flag(flags, FLAG_SUBTRACT, true);
    set_flag(flags, FLAG_HALF_CARRY, half_carry);
    set_flag(flags, FLAG_CARRY, carry);
}
