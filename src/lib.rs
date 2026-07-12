mod constants;
mod globals;
mod hooks;
mod logger;
mod parse_args;
mod soc;
mod utils;

mod debug;

pub mod config;
pub mod emulator;
pub mod init;

// public re-exports
pub use constants::arch::{BusAddrType, XLen};
pub use hooks::{HookAction, PeriodicalTaskArgs};
pub use soc::memory::MemoryValue;
pub use soc::memory_bus::Bus;
pub use soc::peripherals::common::device::{Device, DeviceIOError};
