use crate::*;

/// Width of the tile in pixels.
pub const TILE_WIDTH: usize = 8;
/// Height of the tile in pixels.
pub const TILE_HEIGHT: usize = 8;

pub const PIXEL_SIZE_BITS: usize = 2;
pub const PIXELS_PER_BYTE: usize = 8 / PIXEL_SIZE_BITS;
pub const TILE_PIXEL_COUNT: usize = TILE_WIDTH * TILE_HEIGHT;
/// Size of a tile in **bytes**.
pub const TILE_SIZE: usize = TILE_PIXEL_COUNT / PIXELS_PER_BYTE;

/// Represents tile data of 8x8 pixels.
///
/// Check [VRAM Tile Data](https://gbdev.io/pandocs/Tile_Data.html) for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub data: [u8; TILE_SIZE],
    /// BG Map Attributes (CGB Mode only)
    pub attributes: u8,
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}

impl Tile {
    pub fn new() -> Self {
        Self {
            data: [0; TILE_SIZE],
            attributes: 0,
        }
    }

    pub fn read(emulator: &Emulator, address: u16) -> Self {
        let mut data = [0; TILE_SIZE];

        for i in 0..TILE_SIZE {
            data[i] = emulator.get(address + i as u16);
        }

        Self {
            data,
            attributes: 0,
        }
    }

    /// Read object tile ($8000–$8FFF range in memory)
    pub fn read_object_tile(emulator: &Emulator, tile_index: u8) -> Self {
        let address =
            MEMORY_RANGE_OBJECT_TILES.start as u16 + (tile_index as u16 * TILE_SIZE as u16);
        Self::read(emulator, address)
    }

    /// Read background or window tile using u8 index
    ///
    /// There are 3 banks of tiles. Based on 4th bit of LCDC register, the
    /// tile data can be located at $8000-$87FF or $8800-$97FF.
    pub fn read_bg_tile(emulator: &Emulator, tile_index: u8) -> Self {
        let tile_index = tile_index as u16;
        let address = if emulator.get_bg_win_tiles() {
            MEMORY_RANGE_TILES_BLOCK0.start as u16 + (tile_index * TILE_SIZE as u16)
        } else if tile_index < 128 {
            MEMORY_RANGE_TILES_BLOCK2.start as u16 + (tile_index * TILE_SIZE as u16)
        } else {
            MEMORY_RANGE_TILES_BLOCK1.start as u16 + ((tile_index - 128) * TILE_SIZE as u16)
        };

        Self::read(emulator, address)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> PaletteIndex {
        assert!(x < TILE_WIDTH, "X coordinate out of bounds: {}", x);
        assert!(y < TILE_HEIGHT, "Y coordinate out of bounds: {}", y);

        let shift = TILE_WIDTH - x - 1;

        let bit0 = self.data[y * 2] >> shift & 0b1 == 1;
        let bit1 = self.data[y * 2 + 1] >> shift & 0b1 == 1;

        match (bit1, bit0) {
            (false, false) => PaletteIndex::I0,
            (false, true) => PaletteIndex::I1,
            (true, false) => PaletteIndex::I2,
            (true, true) => PaletteIndex::I3,
        }
    }

    pub fn pixels(&self) -> [PaletteIndex; TILE_PIXEL_COUNT] {
        let mut pixels = [PaletteIndex::I0; TILE_PIXEL_COUNT];

        for y in 0..TILE_HEIGHT {
            for x in 0..TILE_WIDTH {
                let index = y * TILE_WIDTH + x;
                pixels[index] = self.get_pixel(x, y).into();
            }
        }

        pixels
    }
}

/// Color index in the pallet.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum PaletteIndex {
    I0,
    I1,
    I2,
    I3,
}
