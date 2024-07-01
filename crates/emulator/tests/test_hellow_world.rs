use emulator::*;

#[test]
fn test_hello_world() {
    let rom_data = include_bytes!("../../../test-roms/hello.gb");

    let mut emulator = Emulator::from_rom(rom_data.into());

    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0101);

    emulator.handle_next_instruction();
    assert_eq!(emulator.program_counter.0, 0x0150);

    for i in 0..20 {
        let instruction_address = emulator.program_counter.0;
        print!("Opcode: {:02X}", emulator.memory.get(instruction_address));

        let (instruction, size) = emulator.handle_next_instruction();

        match size {
            1 => println!(),
            2 => {
                println!(" {:02X}", emulator.memory.get(instruction_address + 1));
            }
            3 => {
                println!(" {:04X}", emulator.memory.get_u16(instruction_address + 1));
            }
            _ => panic!("Invalid instruction size: {}", size),
        }

        println!("Instruction: {:?}", instruction);
        println!("Step: {}", i);
        println!("Cycles: {}", emulator.cycles);
        println!("PC: {:04X}", emulator.program_counter.0);
        println!("SP: {:04X}", emulator.stack_pointer.0);
        println!("IME: {}", emulator.ime_flag);
        println!(
            "Register AF {:04X}",
            emulator.accumulator_and_flags.as_u16()
        );
        println!("Register BC {:04X}", emulator.register_bc.as_u16());
        println!("Register DE {:04X}", emulator.register_de.as_u16());
        println!("Register HL {:04X}", emulator.register_hl.as_u16());

        let tiles = emulator.get_background_tiles();

        let mut rendered_tile_map = String::new();
        let mut has_tiles = false;
        for y in 0..32 {
            for x in 0..32 {
                let tile = tiles[y * 32 + x];

                if tile == Tile::default() {
                    rendered_tile_map.push_str("  ");
                } else {
                    rendered_tile_map.push_str("██");
                    has_tiles = true;
                }
            }

            rendered_tile_map.push_str("\n");
        }

        if has_tiles {
            println!("{}", rendered_tile_map);
            break;
        } else {
            println!("No bg tiles to render");
        }

        print!("\n");
    }
}
