use emulator::*;

#[test]
fn test_hello_world() {
    let rom_data = include_bytes!("../../../test-roms/hello.gb");

    let mut emulator = Emulator::from_rom(rom_data.into());

    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0101);

    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0150);

    for i in 0..1000 {
        println!(
            "Next opcode: {:02X}",
            emulator.memory.get(emulator.program_counter.into())
        );
        emulator.handle_next_instruction();
        println!("step: {}", i);
        println!("\tCycles: {}", emulator.cycles);
        println!("\tPC: {:04X}", emulator.program_counter.0);
        println!("\tSP: {:04X}", emulator.stack_pointer.0);
        println!("\tIME: {}", emulator.ime_flag);
    }
}
