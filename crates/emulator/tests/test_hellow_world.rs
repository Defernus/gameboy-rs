use emulator::*;

#[test]
fn test_hello_world() {
    let rom_data = include_bytes!("../../../test-roms/hello.gb");

    let mut emulator = Emulator::from_rom(rom_data);

    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0101);

    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0150);

    // TODO add render test
}
