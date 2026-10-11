//! The hardware: system-on-chip (SoC), harts, bus, memory, pipeline, devices.

#![forbid(unsafe_code)]

pub mod config;
pub mod constants;
pub mod globals;
pub mod soc;

mod print_instructions;
mod utils;

// use globals::exit_codes::ExitCode;
