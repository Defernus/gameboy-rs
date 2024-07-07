use emulator::*;
use macroquad::prelude::*;
use runner::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emulator".into(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // let rom = include_bytes!("../../../roms/tetris.gb");
    let rom = include_bytes!("../../../test-roms/hello.gb");

    let mut state = AppState {
        emulator: Emulator::from_rom(rom.to_vec()),
        ..Default::default()
    };

    let mut background_image = Image::gen_image_color(
        TILE_MAP_WIDTH as u16 * TILE_WIDTH as u16,
        TILE_MAP_HEIGHT as u16 * TILE_HEIGHT as u16,
        MAGENTA,
    );
    let background_texture = Texture2D::from_image(&background_image);

    let mut window_image = Image::gen_image_color(
        TILE_MAP_WIDTH as u16 * TILE_WIDTH as u16,
        TILE_MAP_HEIGHT as u16 * TILE_HEIGHT as u16,
        MAGENTA,
    );
    let window_texture = Texture2D::from_image(&window_image);

    let mut mode_3_duration = MODE_3_BASE_DURATION;

    loop {
        if is_key_pressed(KeyCode::R) {
            state.emulator = Emulator::from_rom(rom.to_vec());
        }

        if is_key_pressed(KeyCode::Up) {
            mode_3_duration += 1;
            state.emulator = Emulator::from_rom(rom.to_vec());
            state.emulator.mode_3_duration = mode_3_duration;
            println!("Mode 3 duration: {}", state.emulator.mode_3_duration);
        }
        if is_key_pressed(KeyCode::Down) {
            mode_3_duration -= 1;
            state.emulator = Emulator::from_rom(rom.to_vec());
            state.emulator.mode_3_duration = mode_3_duration;
            println!("Mode 3 duration: {}", state.emulator.mode_3_duration);
        }

        state.emulator.next_frame();

        draw_tilemap_to_image(&state.emulator, &mut background_image, false);
        draw_tilemap_to_image(&state.emulator, &mut window_image, true);

        clear_background(WHITE);

        background_texture.update(&background_image);
        draw_texture_ex(
            &background_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: state.screen_size().into(),
                ..Default::default()
            },
        );

        window_texture.update(&window_image);
        draw_texture_ex(
            &window_texture,
            256.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: state.screen_size().into(),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
