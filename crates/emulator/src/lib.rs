// TODO add support for big-endian targets
#[cfg(target_endian = "big")]
compile_error!("This crate does not support big-endian targets");

pub use emulator_derive::*;

mod control_registers;
mod emulator;
mod flags;
mod instruction_set;
mod memory;
mod program_counter;
mod register;
mod rendering;
mod rom;
mod stack_handlers;
mod stack_pointer;

pub use control_registers::*;
pub use emulator::*;
pub use flags::*;
pub use instruction_set::*;
pub use memory::*;
pub use program_counter::*;
pub use register::*;
pub use rendering::*;
pub use rom::*;
pub use stack_handlers::*;
pub use stack_pointer::*;
