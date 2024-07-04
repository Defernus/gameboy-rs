use emulator::*;

const TEST_TILE: [u8; 16] = [
    0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C,
];

#[test]
fn test_background() {
    let mut emulator = Emulator::default();

    for (i, tile_byte) in TEST_TILE.iter().enumerate() {
        let address = MEMORY_RANGE_TILES_BLOCK2.start + i;
        emulator.set(address as u16, *tile_byte);
    }

    for (i, tile_byte) in TEST_TILE.iter().enumerate() {
        let address = MEMORY_RANGE_TILES_BLOCK2.start + i;
        emulator.set(address as u16, *tile_byte);
    }

    for index in MEMORY_RANGE_TILE_INDICES_BANK0 {
        emulator.set(index as u16, 123);
    }

    let tile_index = 3;
    emulator.set(MEMORY_RANGE_TILE_INDICES_BANK0.start as u16 + tile_index, 0);

    let background = emulator.get_background_tiles();

    assert_eq!(background[0], Tile::default());
    assert_eq!(background[1], Tile::default());
    assert_eq!(background[2], Tile::default());

    assert_eq!(
        background[3],
        Tile {
            data: TEST_TILE,
            ..Default::default()
        }
    );

    assert_eq!(background[4], Tile::default());

    assert_eq!(background[3].get_pixel(0, 0) as u8, 0b00);
    assert_eq!(background[3].get_pixel(1, 0) as u8, 0b10);
    assert_eq!(background[3].get_pixel(1, 5) as u8, 0b01);
    assert_eq!(background[3].get_pixel(1, 1) as u8, 0b11);
}
