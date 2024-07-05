use crate::*;

/// STAT: LCD status
#[derive(ControlRegister)]
#[register(address = 0xFF41)]
pub struct RegisterSTAT(pub u8);

impl RegisterSTAT {
    // TODO Implement getters and setters
}

/// LY indicates the current horizontal line, which might be about to be drawn,
/// being drawn, or just been drawn. LY can hold any value from 0 to 153, with
/// values from 144 to 153 indicating the VBlank period.
#[derive(ControlRegister)]
#[register(address = 0xFF44)]
pub struct RegisterLY(pub u8);

/// The Game Boy constantly compares the value of the LYC and LY registers.
/// When both values are identical, the “LYC=LY” flag in the STAT register
/// is set, and (if enabled) a STAT interrupt is requested.
#[derive(ControlRegister)]
#[register(address = 0xFF45)]
pub struct RegisterLYC(pub u8);
