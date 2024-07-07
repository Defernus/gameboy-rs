use crate::*;
use bit_flag::{bit_flag, flag_mask};

/// LCD Control Register
#[derive(Copy, Clone, ControlRegister)]
#[register(address = 0xFF40)]
pub struct RegisterLCDC(pub u8);

impl Default for RegisterLCDC {
    fn default() -> Self {
        RegisterLCDC(0x91)
    }
}

#[bit_flag]
impl RegisterLCDC {
    /// Controls whether the LCD is on and the PPU is active. Setting it to 0
    /// turns both off, which grants immediate and full access to VRAM, OAM, etc.
    #[flag_mask]
    pub const LCD_AND_PPU_ENABLE: u8 = 0b1000_0000;

    /// Controls which background map the Window uses for rendering.
    /// If `false`, the $9800 tilemap is used, otherwise the $9C00.
    #[flag_mask]
    pub const WIN_TILE_MAP: u8 = 0b0100_0000;

    /// Controls whether the window shall be displayed or not. This flag is
    /// overridden on DMG by bit 0 (bg_and_window_enable) if that bit is clear.
    ///
    /// Check [documentation](https://gbdev.io/pandocs/LCDC.html#lcdc5--window-enable)
    /// for more details
    #[flag_mask]
    pub const WINDOW_ENABLE: u8 = 0b0010_0000;

    /// Controls which addressing mode the BG and Window use to pick tiles.
    ///
    /// Objects (sprites) aren’t affected by this, and will always use the
    /// $8000 addressing mode.
    #[flag_mask]
    pub const BG_AND_WIN_TILE: u8 = 0b0001_0000;

    /// THis flag works similarly to LCDC bit 6: if the bit is clear (0), the
    /// BG uses tilemap $9800, otherwise tilemap $9C00.
    #[flag_mask]
    pub const BG_TILE_MAP: u8 = 0b0000_1000;

    /// Vontrols the size of all objects (1 tile or 2 stacked vertically).
    #[flag_mask]
    pub const OBJECT_SIZE: u8 = 0b0000_0100;

    /// Toggles whether objects are displayed or not.
    #[flag_mask]
    pub const OBJECT_ENABLE: u8 = 0b0000_0010;

    /// In non CGB mode:
    ///
    /// When this flag is cleared, both background and window become blank
    /// (white), and the Window Display Bit is ignored in that case. Only
    /// objects may still be displayed (if enabled in Bit 1).
    ///
    /// In CGB Mode:
    ///
    /// When this flag is cleared, the background and window lose their
    /// priority - the objects will be always displayed on top of background
    /// and window, independently of the priority flags in OAM and BG Map attributes.
    #[flag_mask]
    pub const BG_AND_WINDOW_ENABLE: u8 = 0b0000_0001;
}
