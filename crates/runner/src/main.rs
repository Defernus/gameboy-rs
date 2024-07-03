use emulator::*;
use macroquad::prelude::*;
use runner::*;

#[macroquad::main("Emulator")]
async fn main() {
    let hello_world_rom = include_bytes!("../../../test-roms/hello.gb");
    let mut emulator = Emulator::from_rom(hello_world_rom.to_vec());

    let mut background_image = Image::gen_image_color(
        TILE_MAP_WIDTH as u16 * TILE_WIDTH as u16,
        TILE_MAP_HEIGHT as u16 * TILE_HEIGHT as u16,
        MAGENTA,
    );
    let background_texture = Texture2D::from_image(&background_image);

    let mut step = 0;

    let instructions_per_frame = 1000;

    loop {
        for _ in 0..instructions_per_frame {
            debug_next_instruction(&mut emulator, step);
            step += 1;
        }

        draw_background_to_image(&emulator, &mut background_image);

        clear_background(BLACK);

        background_texture.update(&background_image);
        draw_texture_ex(
            &background_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: vec2(512.0, 512.0).into(),
                ..Default::default()
            },
        );

        next_frame().await
    }
}

fn debug_next_instruction(emulator: &mut Emulator, step: usize) {
    print!("St: {:09} ", step);

    print!("Op {:02X} ", emulator.instruction_register);

    let instruction = emulator.handle_next_instruction();

    print!("Cy {} ", emulator.cycles);
    print!("PC {:04X} ", emulator.program_counter.0);
    print!("SP {:04X} ", emulator.stack_pointer.0);
    print!("IME {} ", emulator.ime_flag);
    print!("AF {:04X} ", emulator.accumulator_and_flags.as_u16());
    print!("BC {:04X} ", emulator.register_bc.as_u16());
    print!("DE {:04X} ", emulator.register_de.as_u16());
    print!("HL {:04X} ", emulator.register_hl.as_u16());

    println!("{:?}", instruction);
}
