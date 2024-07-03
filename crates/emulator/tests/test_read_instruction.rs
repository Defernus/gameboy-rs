use emulator::{ArgumentN16, ArgumentR16, Emulator, Instruction, InstructionLD, InstructionNOP};

#[test]
fn test_read_instruction() {
    let mut emulator = Emulator::default();

    emulator.memory.set(0x0150, 0x31); // LD sp, n16

    // n16 = 0x1234, stored in little-endian format
    emulator.memory.set(0x0151, 0x34);
    emulator.memory.set(0x0152, 0x12);
    emulator.memory.set(0x0153, 0x10);

    emulator.program_counter = 0x0150.into();

    // first instruction is always NOP because during it CPU reads the opcode
    // of the actual first instruction
    let instruction = emulator.handle_next_instruction();
    assert_eq!(instruction, InstructionNOP.into());

    assert_eq!(emulator.program_counter.0, 0x0151);

    let instruction = emulator.handle_next_instruction();
    let expected_instruction =
        Instruction::LD(InstructionLD::R16_N16(ArgumentR16::SP, ArgumentN16(0x1234)));

    assert_eq!(instruction, expected_instruction);

    // Next instruction already loaded to IR register and PC incremented
    assert_eq!(emulator.instruction_register, 0x10);
    assert_eq!(emulator.program_counter.0, 0x0154);
}
