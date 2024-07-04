use crate::*;

/// Load instructions. These instructions are used to move data from
/// register/value/memory on the right to location on the left.
///
/// Example: `LD B, C` copies the value in register `C` to register `B`.
///
/// Flags not affected except for `HL_SP_E8` (Check variant documentation for details).
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionLD {
    R8_R8(ArgumentR8, ArgumentR8),
    R8_N8(ArgumentR8, ArgumentN8),
    R16_N16(ArgumentR16, ArgumentN16),
    AtR16_A(ArgumentR16),
    AtN16_A(ArgumentN16),
    A_AtR16(ArgumentR16),
    A_AtN16(ArgumentN16),
    AtHLI_A,
    AtHLD_A,
    A_AtHLI,
    A_AtHLD,
    /// Store SP & $FF at address n16 and SP >> 8 at address n16 + 1.
    AtN16_SP(ArgumentN16),
    /// Store SP + e8 at HL.
    ///
    /// Flags:
    ///
    /// | Flag | Value |
    /// |------|-------|
    /// | Z    | 0     |
    /// | N    | 0     |
    /// | H    | Set if overflow from bit 3. |
    /// | C    | Set if overflow from bit 7. |
    HL_SP_E8(ArgumentE8),
    SP_HL,
}

impl InstructionTrait for InstructionLD {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        match *self {
            Self::R8_R8(to, from) => {
                *to.get_mut(emulator) = from.get(emulator);

                match (to, from) {
                    // impossible
                    (ArgumentR8::AtHL, ArgumentR8::AtHL) => 0,
                    (ArgumentR8::AtHL, _) => 2,
                    (_, ArgumentR8::AtHL) => 2,
                    _ => 1,
                }
            }
            Self::R8_N8(to, from) => {
                *to.get_mut(emulator) = from.get(emulator);

                match to {
                    ArgumentR8::AtHL => 3,
                    _ => 2,
                }
            }
            Self::R16_N16(to, from) => {
                *to.get_mut(emulator) = from.get(emulator);

                3
            }
            Self::AtR16_A(to) => {
                *to.at_mut(emulator) = emulator.accumulator_and_flags.high();

                2
            }
            Self::AtN16_A(to) => {
                *to.at_mut(emulator) = emulator.accumulator_and_flags.high();

                4
            }
            Self::A_AtR16(from) => {
                emulator.accumulator_and_flags.set_high(from.at(emulator));

                2
            }
            Self::A_AtN16(from) => {
                emulator.accumulator_and_flags.set_high(from.at(emulator));

                4
            }
            Self::AtHLI_A => {
                *emulator.register_hl.at_mut(emulator) = emulator.accumulator_and_flags.high();
                emulator.register_hl.increment();

                2
            }
            Self::AtHLD_A => {
                *emulator.register_hl.at_mut(emulator) = emulator.accumulator_and_flags.high();
                emulator.register_hl.decrement();

                2
            }
            Self::A_AtHLI => {
                emulator
                    .accumulator_and_flags
                    .set_high(emulator.register_hl.at(emulator));
                emulator.register_hl.increment();

                2
            }
            Self::A_AtHLD => {
                emulator
                    .accumulator_and_flags
                    .set_high(emulator.register_hl.at(emulator));
                emulator.register_hl.decrement();

                2
            }
            Self::AtN16_SP(to) => {
                let value: u16 = emulator.stack_pointer.into();
                let address = to.get(emulator);

                emulator.set(address, (value & 0xFF) as u8);
                emulator.set(address.wrapping_add(1), (value >> 8) as u8);

                5
            }
            Self::HL_SP_E8(offset) => {
                let flags = emulator.accumulator_and_flags.low_mut();
                let value = offset.apply_offset_with_flags(emulator.stack_pointer.into(), flags);

                set_flag(flags, FLAG_ZERO, false);
                set_flag(flags, FLAG_SUBTRACT, false);

                *emulator.register_hl.as_u16_mut() = value;

                3
            }
            Self::SP_HL => {
                emulator.stack_pointer = emulator.register_hl.as_u16().into();

                2
            }
        }
    }
}
