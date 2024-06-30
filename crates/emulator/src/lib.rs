// TODO add support for big-endian targets
#[cfg(target_endian = "big")]
compile_error!("This crate does not support big-endian targets");

mod emulator;
mod flags;
mod instruction_set;
mod memory;
mod program_counter;
mod register;
mod rom;
mod stack_handlers;
mod stack_pointer;

pub use emulator::*;
pub use flags::*;
pub use instruction_set::*;
pub use memory::*;
pub use program_counter::*;
pub use register::*;
pub use rom::*;
pub use stack_handlers::*;
pub use stack_pointer::*;
