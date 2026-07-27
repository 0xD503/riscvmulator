#![forbid(unsafe_code)]

mod hooks;
mod logger;

mod debug;

pub mod emulator;
pub mod init;

// public re-exports
pub use hooks::{HookAction, PeriodicalTaskArgs};
pub use riscvmulator_soc::constants::arch::{BusAddrType, Byte, Word, XLen};
pub use riscvmulator_soc::globals::exit_codes::ExitCode;
pub use riscvmulator_soc::soc::memory::MemoryValueType;
pub use riscvmulator_soc::soc::memory_bus::Bus;
pub use riscvmulator_soc::soc::peripherals::common::device::{
    Addressable, Device, DeviceMeta, IODeviceError,
};
