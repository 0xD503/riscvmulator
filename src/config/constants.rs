use crate::constants::arch::XLen;

pub const DEFAULT_ENABLE_EXTENSION_I: bool = true;
pub const DEFAULT_ENABLE_EXTENSION_M: bool = false;
pub const DEFAULT_ENABLE_EXTENSION_A: bool = false;
pub const DEFAULT_ENABLE_EXTENSION_F: bool = false;
pub const DEFAULT_ENABLE_EXTENSION_D: bool = false;
pub const DEFAULT_ENABLE_EXTENSION_C: bool = false;
pub const DEFAULT_ENABLE_EXTENSION_ZIFENCEI: bool = false;
pub const DEFAULT_ENABLE_EXTENSION_ZICSR: bool = false;

pub const DEFAULT_HARTS_NUM: u32 = 1;
pub const DEFAULT_XTAL_FREQ_HZ: u64 = 16384;
pub const DEFAULT_PC_RESET_VALUE: XLen = DEFAULT_RAM_BASE;
pub const DEFAULT_RAM_BASE: XLen = 0x8000_0000;
pub const DEFAULT_MEMORY_SIZE: usize = 65536; // 64 KB
pub const DEFAULT_RAM_END: XLen =
    DEFAULT_RAM_BASE + DEFAULT_MEMORY_SIZE as XLen;

pub const DEFAULT_LOG_LEVEL: &str = "info";
pub const DEFAULT_THROTTLE_FREQ: u64 = 0;

pub const CONFIG_ENV_VAR_PREFIX: &str = "RVMULATOR_";
