use crate::*;
use emulator::*;
use macroquad::prelude::*;

pub struct AppDebugState {
    /// Amount of steps to run when the "Run" button is pressed
    pub steps: usize,

    /// csv file to store all instructions executed
    pub instructions_debug_file: Option<String>,
}

impl AppDebugState {
    pub fn init(&self) {
        if let Some(debug_file) = &self.instructions_debug_file {
            init_debug_table(debug_file);
        }
    }
}

impl Default for AppDebugState {
    fn default() -> Self {
        Self {
            steps: 1,
            instructions_debug_file: None,
        }
    }
}

pub fn draw_debug_ui(state: &mut AppState) {
    let screen_size = state.screen_size();
    draw_tile_indices(
        &state.emulator,
        0.0,
        0.0,
        screen_size.x / TILE_MAP_WIDTH as f32,
        screen_size.x / TILE_MAP_HEIGHT as f32,
    );

    if is_key_down(KeyCode::Up) {
        state.debug_state.steps += (state.debug_state.steps / 10).max(1);
        println!("Steps inc: {}", state.debug_state.steps);
    }
    if is_key_down(KeyCode::Down) && state.debug_state.steps > 1 {
        state.debug_state.steps -= (state.debug_state.steps / 10).max(1);
        println!("Steps dec: {}", state.debug_state.steps);
    }

    if is_key_pressed(KeyCode::Space) {
        for _ in 0..state.debug_state.steps {
            if let Some(debug_file) = &state.debug_state.instructions_debug_file {
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .open(debug_file)
                    .unwrap();

                debug_next_instruction(&mut state.emulator, &mut file, state.steps);
            } else {
                state.emulator.handle_next_instruction();
            }
            state.steps += 1;
        }
        println!("Total steps: {}", state.steps);
    }
}
