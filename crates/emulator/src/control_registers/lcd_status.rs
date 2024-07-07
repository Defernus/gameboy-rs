use crate::*;
use bit_flag::{bit_flag, flag_mask, value_mask, U2};

/// STAT: LCD status
#[derive(Debug, Copy, Clone, ControlRegister)]
#[register(address = 0xFF41)]
pub struct RegisterSTAT(pub u8);

impl Default for RegisterSTAT {
    fn default() -> Self {
        RegisterSTAT(0x85)
    }
}

#[bit_flag]
impl RegisterSTAT {
    /// **LYC int select**: If set, selects the `LYC == LY` condition for the
    /// STAT interrupt.
    #[flag_mask]
    pub const LYC_INT_SELECT: u8 = 0b0100_0000;

    /// **Mode 2 int select**: If set, selects the Mode 2 condition for the STAT interrupt.
    #[flag_mask]
    pub const MODE_2_INT_SELECT: u8 = 0b0010_0000;

    /// **Mode 1 int select**: If set, selects the Mode 1 condition for the STAT interrupt.
    #[flag_mask]
    pub const MODE_1_INT_SELECT: u8 = 0b0001_0000;

    /// **Mode 0 int select**: If set, selects the Mode 0 condition for the STAT interrupt.
    #[flag_mask]
    pub const MODE_0_INT_SELECT: u8 = 0b0000_1000;

    /// **LYC == LY**: This bit is set when the LYC register equals the LY register.
    #[flag_mask]
    pub const LYC_EQUALS_LY: u8 = 0b0000_0100;

    /// Indicates the PPU’s (Pixel-Processing Unit) current status.
    ///
    /// Check [documentation](https://gbdev.io/pandocs/Rendering.html#ppu-modes) for more details.
    #[value_mask(PpuMode)]
    pub const PPU_MODE: u8 = 0b0000_0011;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum PpuMode {
    /// Waiting until the end of the scanline
    ///
    /// Duration: 376 - PPU_MODE_3 duration
    Mode0,
    /// Waiting until the next frame
    ///
    /// Duration: 4560 dots (10 scanlines)
    Mode1,
    /// Searching for OBJs which overlap this line
    ///
    /// Duration: 80 dots
    Mode2,
    /// Sending pixels to the LCD
    ///
    /// Duration: Between 172 and 289 dots
    Mode3,
}

impl From<U2> for PpuMode {
    fn from(value: U2) -> Self {
        match value.to_bits() {
            [false, false] => PpuMode::Mode0,
            [true, false] => PpuMode::Mode1,
            [false, true] => PpuMode::Mode2,
            [true, true] => PpuMode::Mode3,
        }
    }
}

impl From<PpuMode> for U2 {
    fn from(value: PpuMode) -> Self {
        match value {
            PpuMode::Mode0 => U2::new(0),
            PpuMode::Mode1 => U2::new(1),
            PpuMode::Mode2 => U2::new(2),
            PpuMode::Mode3 => U2::new(3),
        }
    }
}

/// LY indicates the current horizontal line, which might be about to be drawn,
/// being drawn, or just been drawn. LY can hold any value from 0 to 153, with
/// values from 144 to 153 indicating the VBlank period.
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF44)]
pub struct RegisterLY(pub u8);

/// The Game Boy constantly compares the value of the LYC and LY registers.
/// When both values are identical, the “LYC=LY” flag in the STAT register
/// is set, and (if enabled) a STAT interrupt is requested.
#[derive(Default, ControlRegister, Clone, Copy)]
#[register(address = 0xFF45)]
pub struct RegisterLYC(pub u8);
