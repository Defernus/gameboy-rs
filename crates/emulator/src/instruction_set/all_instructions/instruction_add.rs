use crate::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum InstructionADD {
    /// Add the value in r8 to A.
    ///
    /// Flags:
    ///
    /// | Flag | Value |
    /// |------|-------|
    /// | Z    | Set if result is 0. |
    /// | N    | 0 |
    /// | H    | Set if carry from bit 3. |
    /// | C    | Set if carry from bit 7. |
    A_R8(ArgumentR8),
    /// Add the value in n8 to A.
    ///
    /// Flags same as A_R8.
    A_N8(ArgumentN8),
    /// Add the value in r16 to HL.
    ///
    /// Flags:
    ///
    /// | Flag | Value |
    /// |------|-------|
    /// | N    | 0 |
    /// | H    | Set if carry from bit 11. |
    /// | C    | Set if carry from bit 15. |
    HL_R16(ArgumentR16),
    /// Add the signed value e8 to SP.
    ///
    /// Flags:
    ///
    /// | Flag | Value |
    /// |------|-------|
    /// | Z    | 0 |
    /// | N    | 0 |
    /// | H    | Set if carry from bit 3. |
    /// | C    | Set if carry from bit 7. |
    SP_E8(ArgumentE8),
}

impl InstructionTrait for InstructionADD {
    fn execute(&self, emulator: &mut Emulator) -> u8 {
        match self {
            Self::A_R8(input) => {
                exec_add_to_a(emulator, input.get(emulator));

                if *input == ArgumentR8::AtHL {
                    2
                } else {
                    1
                }
            }
            Self::A_N8(input) => {
                exec_add_to_a(emulator, input.get(emulator));

                2
            }
            Self::HL_R16(inout) => {
                exec_add_to_hl(emulator, inout.get(emulator));

                2
            }
            Self::SP_E8(input) => {
                let stack_pointer: u16 = emulator.stack_pointer.into();
                let result = input.apply_offset(stack_pointer);

                let input_u16 = input.get(emulator) as u16;

                let flags = emulator.accumulator_and_flags.low_mut();

                set_flag(flags, ZERO_FLAG, false);
                set_flag(flags, SUBTRACT_FLAG, false);

                update_carry_flags_add_u8(
                    flags,
                    (stack_pointer & 0xFF) as u8,
                    (input_u16 & 0xFF) as u8,
                );

                emulator.stack_pointer = result.into();

                4
            }
        }
    }
}

fn exec_add_to_a(emulator: &mut Emulator, value: u8) {
    let register_a = emulator.accumulator_and_flags.high();
    let result = register_a.wrapping_add(value);
    emulator.accumulator_and_flags.set_high(result);

    let flags = emulator.accumulator_and_flags.low_mut();

    set_flag(flags, ZERO_FLAG, result == 0);
    set_flag(flags, SUBTRACT_FLAG, false);
    update_carry_flags_add_u8(flags, register_a, value);
}

fn exec_add_to_hl(emulator: &mut Emulator, value: u16) {
    let register_hl = emulator.register_hl.as_u16();
    let result = register_hl.wrapping_add(value);
    emulator.register_hl.set(result);

    let flags = emulator.accumulator_and_flags.low_mut();

    set_flag(flags, SUBTRACT_FLAG, false);

    update_carry_flags_add_u16(flags, register_hl, value);
}
