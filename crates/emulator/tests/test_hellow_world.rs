use emulator::*;

#[test]
fn test_hello_world() {
    let rom_data = include_bytes!("../../../test-roms/hello.gb");

    let mut emulator = Emulator::from_rom(rom_data);

    // Skip first NOP from initial state of the instruction register (IR)
    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0101);

    // Handling actual first instruction (NOP)
    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0102);

    // Handling second instruction (JP $0150)
    emulator.handle_next_instruction();
    // PC is 0x0151 because CPU already read the opcode of the next
    // instruction and stored it to IR
    assert_eq!(emulator.program_counter.0, 0x0151);

    // TODO add render test
}
