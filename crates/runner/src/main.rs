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
    let hello_world_rom = include_bytes!("../../../test-roms/hello.gb");
    // let hello_world_rom = include_bytes!("../../../roms/tetris.gb");
    let mut state = AppState {
        emulator: Emulator::from_rom(hello_world_rom.to_vec()),
        debug_state: AppDebugState {
            instructions_debug_file: Some("debug.csv".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    state.init();

    let mut background_image = Image::gen_image_color(
        TILE_MAP_WIDTH as u16 * TILE_WIDTH as u16,
        TILE_MAP_HEIGHT as u16 * TILE_HEIGHT as u16,
        MAGENTA,
    );
    let background_texture = Texture2D::from_image(&background_image);

    loop {
        draw_background_to_image(&state.emulator, &mut background_image);

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

        draw_debug_ui(&mut state);

        next_frame().await
    }
}
