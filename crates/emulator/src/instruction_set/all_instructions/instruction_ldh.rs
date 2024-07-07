use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionLDH {
    /// Store value in register A into the byte at address $FF00+n8.
    AtN8_A(ArgumentN8),
    /// Store value in register A into the byte at address $FF00+c.
    AtC_A,
    /// Load value in register A from the byte at address $FF00+n8.
    A_AtN8(ArgumentN8),
    /// Load value in register A from the byte at address $FF00+c.
    A_AtC,
}

impl InstructionTrait for InstructionLDH {
    fn execute(&self, emulator: &mut Emulator) -> usize {
        match self {
            Self::AtN8_A(to) => {
                emulator.set(
                    0xFF00 + to.get(emulator) as u16,
                    emulator.accumulator_and_flags.high(),
                );

                3
            }
            Self::AtC_A => {
                emulator.set(
                    0xFF00 + emulator.register_bc.low() as u16,
                    emulator.accumulator_and_flags.high(),
                );

                2
            }
            Self::A_AtN8(from) => {
                let value = emulator.get(0xFF00 + from.get(emulator) as u16);
                emulator.accumulator_and_flags.set_high(value);

                3
            }
            Self::A_AtC => {
                emulator
                    .accumulator_and_flags
                    .set_high(emulator.get(0xFF00 + emulator.register_bc.low() as u16));

                2
            }
        }
    }
}
