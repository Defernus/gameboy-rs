use crate::*;

/// Amount of DOTs per M-cycle.
pub const DOTS_PER_M_CYCLE: usize = 4;

/// Amount of M-cycles per M-cycle in double speed mode.
pub const DOTS_PER_M_CYCLE_DOUBLE_SPEED: usize = 2;

pub const SCANLINE_DURATION: usize = 456;

pub const TOTAL_SCANLINES: usize = 154;
pub const LAST_SCANLINE: usize = TOTAL_SCANLINES - 1;

pub const MODE_0_BASE_DURATION: usize = 376;
pub const MODE_1_DURATION: usize = SCANLINE_DURATION * 10;
pub const MODE_2_DURATION: usize = 80;
// pub const MODE_3_BASE_DURATION: usize = 190;
pub const MODE_3_BASE_DURATION: usize = 172;

impl Emulator {
    /// Handle single dot (smallest unit of time in Pixel Processing Unit).
    pub fn handle_dot(&mut self) {
        if !self.reg::<RegisterLCDC>().get_lcd_and_ppu_enable() {
            // skip if PPU is disabled
            return;
        }

        let current_mode = self.reg::<RegisterSTAT>().get_ppu_mode();
        self.scanline_progress += 1;
        self.dots_in_current_mode += 1;

        assert!(
            self.scanline_progress <= SCANLINE_DURATION,
            "Mode 0 duration exceeded: {}",
            self.scanline_progress
        );

        let is_scanline_ended = self.scanline_progress == SCANLINE_DURATION;
        let current_scanline = self.reg::<RegisterLY>().0 as usize;

        let new_mode = match current_mode {
            // Searching for OBJs which overlap this line
            PpuMode::Mode2 => {
                assert!(
                    self.scanline_progress <= MODE_2_DURATION,
                    "scanline progress exceeded ppu mode 2: {}",
                    self.scanline_progress,
                );

                if self.scanline_progress == MODE_2_DURATION {
                    PpuMode::Mode3.into()
                } else {
                    None
                }
            }
            // Sending pixels to the LCD
            PpuMode::Mode3 => {
                if self.scanline_progress >= MODE_2_DURATION + self.mode_3_duration {
                    PpuMode::Mode0.into()
                } else {
                    None
                }
            }
            // Waiting until the end of the scanline
            PpuMode::Mode0 => {
                if is_scanline_ended {
                    self.reg_mut::<RegisterLY>().increment();

                    assert!(
                        current_scanline < SCREEN_HEIGHT,
                        "ppu mode 0 at wrong scanline {current_scanline}"
                    );
                    if current_scanline + 1 == SCREEN_HEIGHT {
                        // screen end reached
                        self.is_frame_available = true;
                        PpuMode::Mode1.into()
                    } else {
                        // scanline ended, start new one
                        PpuMode::Mode2.into()
                    }
                } else {
                    None
                }
            }
            // VBlank (Waiting until the next frame)
            PpuMode::Mode1 => {
                if self.dots_in_current_mode >= MODE_1_DURATION {
                    self.reg_mut::<RegisterLY>().0 = 0;
                    PpuMode::Mode2.into()
                } else {
                    None
                }
            }
        };

        if is_scanline_ended {
            self.scanline_progress = 0;
        }

        if let Some(new_mode) = new_mode {
            self.reg_mut::<RegisterSTAT>().set_ppu_mode(new_mode);
            self.dots_in_current_mode = 0;
        }
    }

    pub fn dots_per_cycle(&self) -> usize {
        if self.double_speed {
            DOTS_PER_M_CYCLE_DOUBLE_SPEED
        } else {
            DOTS_PER_M_CYCLE
        }
    }

    /// Handle amount of dots in single M-cycle (4 dots in normal speed, 2 dots in double speed)
    pub fn handle_dots_in_cycle(&mut self) {
        let dots_to_handle = self.dots_per_cycle();

        for _ in 0..dots_to_handle {
            self.handle_dot();
        }
    }

    /// Run emulator until next is available.
    pub fn next_frame(&mut self) {
        while !self.is_frame_available {
            self.handle_next_instruction();
        }

        self.is_frame_available = false;
    }
}
