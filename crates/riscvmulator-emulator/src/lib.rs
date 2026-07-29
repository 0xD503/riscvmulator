#![forbid(unsafe_code)]

mod debug;
mod emulator;
mod globals;
mod hooks;
mod logger;
mod parse_args;
mod utils;

pub mod config;
pub mod init;

// public re-exports
pub use crate::emulator::{Emulator, EmulatorError, constants::EmulatorEvent};
pub use crate::parse_args::CLI_Args;
pub use emulator::elf_loader::{ElfData, ElfError};

pub use hooks::{HookAction, PeriodicalTaskArgs};
pub use riscvmulator_soc::config::SocCfg;
pub use riscvmulator_soc::constants::arch::{BusAddrType, Byte, Word, XLen};
pub use riscvmulator_soc::globals::exit_codes::ExitCode;
pub use riscvmulator_soc::soc::memory::MemoryValueType;
pub use riscvmulator_soc::soc::memory_bus::{Bus, SystemBus};
pub use riscvmulator_soc::soc::peripherals::common::device::{
    Addressable, Device, DeviceMeta, IODeviceError,
};
