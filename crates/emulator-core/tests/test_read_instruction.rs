use emulator_core::{ArgumentN16, ArgumentR16, Emulator, Instruction, InstructionLD};

#[test]
fn test_read_instruction() {
    let mut emulator = Emulator::default();

    emulator.memory.set(0x0150, 0x31); // LD sp, n16

    // n16 = 0x1234, stored in little-endian format
    emulator.memory.set(0x0151, 0x34);
    emulator.memory.set(0x0152, 0x12);

    emulator.program_counter = 0x0150.into();

    let (instruction, size) = emulator.read_next_instruction().unwrap();
    let expected_instruction =
        Instruction::LD(InstructionLD::R16_N16(ArgumentR16::SP, ArgumentN16(0x1234)));

    assert_eq!(size, 3);
    assert_eq!(instruction, expected_instruction);
}
