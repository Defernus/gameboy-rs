use crate::AppDebugState;
use emulator::*;
use macroquad::math::Vec2;

pub struct AppState {
    pub emulator: Emulator,
    /// Number of instructions executed
    pub steps: usize,

    pub screen_scale: f32,

    pub debug_state: AppDebugState,
}

impl AppState {
    pub fn new(emulator: Emulator) -> Self {
        Self {
            emulator,
            steps: 0,
            screen_scale: 3.0,
            debug_state: AppDebugState::default(),
        }
    }

    pub fn init(&mut self) {
        self.debug_state.init();
    }

    /// Get destination screen size
    pub fn screen_size(&self) -> Vec2 {
        let width = self.screen_scale * (TILE_MAP_WIDTH * TILE_WIDTH) as f32;
        let height = self.screen_scale * (TILE_MAP_HEIGHT * TILE_HEIGHT) as f32;

        Vec2::new(width, height)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(Emulator::default())
    }
}
