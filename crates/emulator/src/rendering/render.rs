use crate::*;

/// GBC has 15 bit color depth.
pub const RENDER_PIXEL_SIZE: usize = 2;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const TILE_MAP_WIDTH: usize = 32;
pub const TILE_MAP_HEIGHT: usize = 32;
/// Amount of tiles in the tile map.
pub const TILE_MAP_TILES_COUNT: usize = TILE_MAP_WIDTH * TILE_MAP_HEIGHT;
/// Size of the tile map in bytes.
pub const TILE_MAP_SIZE: usize = TILE_MAP_TILES_COUNT * TILE_SIZE;

impl Emulator {
    pub fn render(&self) -> [u8; SCREEN_SIZE] {
        todo!()
    }

    /// Get index of the tile by it's position in the tile map.
    ///
    /// if window is true, then window tile map is used, otherwise background.
    pub fn get_tile_index(&self, position: usize, window: bool) -> u8 {
        let selected_tilemap_flag = if window {
            self.reg::<RegisterLCDC>().get_win_tile_map()
        } else {
            self.reg::<RegisterLCDC>().get_bg_tile_map()
        };

        let selected_tilemap = if selected_tilemap_flag {
            MEMORY_RANGE_TILE_INDICES_BANK1.start
        } else {
            MEMORY_RANGE_TILE_INDICES_BANK0.start
        };

        let address = selected_tilemap + position;
        self.get(address as u16)
    }

    /// Get tilemap from the memory.
    ///
    /// if window is true, then window tile map is used, otherwise background.
    pub fn get_tiles(&self, window: bool) -> [Tile; TILE_MAP_TILES_COUNT] {
        let mut result = [Tile::default(); TILE_MAP_TILES_COUNT];

        for tile_position in 0..TILE_MAP_TILES_COUNT {
            let tile_index = self.get_tile_index(tile_position, window);
            let tile = Tile::read_bg_tile(self, tile_index);

            // TODO add tile attribute if CGB mode enabled

            result[tile_position] = tile;
        }

        result
    }
}

/// Convert tile map cords to position in the tile map.
pub fn tile_map_cords_to_position(x: usize, y: usize) -> usize {
    y * TILE_MAP_WIDTH + x
}

/// Convert position of the tile in the tile map to its cords.
pub fn tile_map_position_to_cords(position: usize) -> (usize, usize) {
    let x = position % TILE_MAP_WIDTH;
    let y = position / TILE_MAP_WIDTH;

    (x, y)
}
