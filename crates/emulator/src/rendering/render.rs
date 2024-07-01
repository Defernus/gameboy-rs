use crate::*;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
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

    pub fn get_bg_tile_index(&self, position: usize) -> u8 {
        let address = MEMORY_RANGE_TILE_INDICES_BANK0.start + position;
        self.memory.get(address as u16)
    }

    pub fn get_background_tiles(&self) -> [Tile; TILE_MAP_TILES_COUNT] {
        let mut result = [Tile::default(); TILE_MAP_TILES_COUNT];

        for tile_position in 0..TILE_MAP_TILES_COUNT {
            let tile_index = self.get_bg_tile_index(tile_position);
            let tile = Tile::read_bg_tile(self, tile_index);

            // TODO add tile attribute if CGB mode enabled

            result[tile_position] = tile;
        }

        result
    }
}
