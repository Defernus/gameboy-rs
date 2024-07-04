use emulator::*;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 16.0;

pub fn draw_tile_indices(emulator: &Emulator, x: f32, y: f32, tile_width: f32, wile_height: f32) {
    for tile_x in 0..TILE_MAP_WIDTH {
        for tile_y in 0..TILE_MAP_HEIGHT {
            let tile_position = tile_y * TILE_MAP_WIDTH + tile_x;
            let bg_tile_index = emulator.get_bg_tile_index(tile_position);

            let tile_x_offset = x + tile_x as f32 * tile_width;
            let tile_y_offset = y + tile_y as f32 * wile_height;

            let text = format!("{bg_tile_index:02X}");
            draw_text(&text, tile_x_offset, tile_y_offset, FONT_SIZE, MAGENTA);
        }
    }
}
