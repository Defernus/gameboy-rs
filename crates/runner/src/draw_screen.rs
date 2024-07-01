use emulator::*;
use macroquad::prelude::*;

/// Draw emulator screen
pub fn draw_background_to_image(emulator: &Emulator, image: &mut Image) {
    let background = emulator.get_background_tiles();

    for tile_y in 0..TILE_MAP_HEIGHT {
        for tile_x in 0..TILE_MAP_HEIGHT {
            let tile_position = tile_map_cords_to_position(tile_x, tile_y);

            let tile = background[tile_position];

            let tile_x = tile_x as u32 * TILE_WIDTH as u32;
            let tile_y = tile_y as u32 * TILE_HEIGHT as u32;

            draw_tile_to_image(image, &tile, tile_x, tile_y);
        }
    }
}

fn draw_tile_to_image(image: &mut Image, tile: &Tile, x: u32, y: u32) {
    for tile_y in 0..TILE_HEIGHT {
        for tile_x in 0..TILE_WIDTH {
            let color = tile.get_pixel(tile_x, tile_y) as u8 * 64;

            let x = x + tile_x as u32;
            let y = y + tile_y as u32;

            let color = Color::from_rgba(color, color, color, 255);
            image.set_pixel(x, y, color);
        }
    }
}
