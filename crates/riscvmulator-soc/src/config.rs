pub mod constants;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::constants::arch::XLen;

use self::constants::*;

use crate::constants::arch::{BusAddrType, IRQ_Line};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct HartCfg {
    pub(super) pc_reset_value: XLen,

    pub extension_I: bool,
    extension_M: bool,
    extension_A: bool,
    extension_F: bool,
    extension_D: bool,
    extension_C: bool,
    extension_Zifencei: bool,
    extension_Zicsr: bool,
}

impl Default for HartCfg {
    fn default() -> Self {
        Self {
            pc_reset_value: DEFAULT_PC_RESET_VALUE,

            extension_I: DEFAULT_ENABLE_EXTENSION_I,
            extension_M: DEFAULT_ENABLE_EXTENSION_M,
            extension_A: DEFAULT_ENABLE_EXTENSION_A,
            extension_F: DEFAULT_ENABLE_EXTENSION_F,
            extension_D: DEFAULT_ENABLE_EXTENSION_D,
            extension_C: DEFAULT_ENABLE_EXTENSION_C,
            extension_Zifencei: DEFAULT_ENABLE_EXTENSION_ZIFENCEI,
            extension_Zicsr: DEFAULT_ENABLE_EXTENSION_ZICSR,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MemoryCfg {
    pub(super) base_addr: BusAddrType,
    pub size_bytes: usize,
    // #[serde(skip)]
    // ram_end: XLen,
}

impl Default for MemoryCfg {
    fn default() -> Self {
        Self {
            base_addr: DEFAULT_RAM_BASE,
            size_bytes: DEFAULT_MEMORY_SIZE,
            // ram_end: DEFAULT_RAM_END,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeviceCfg {
    pub base_addr: BusAddrType,
    pub size_bytes: usize,
    pub kind: String,
    pub name: String,
    pub irq_line: Option<IRQ_Line>,
}

impl DeviceCfg {
    pub fn base_addr(&self) -> BusAddrType {
        self.base_addr
    }
    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }

    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

// impl Default for DeviceCfg {
//     fn default() -> Self {
//         Self {}
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BusCfg {
    pub memory: MemoryCfg,
    pub devices: Vec<DeviceCfg>,
}

impl Default for BusCfg {
    fn default() -> Self {
        Self {
            memory: MemoryCfg::default(),
            devices: Default::default(), // empty list of devices
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SocCfg {
    pub hart: HartCfg,
    pub bus: BusCfg,
    pub harts_num: u32,
    pub xtal_frequency_hz: u64,
}

impl Default for SocCfg {
    fn default() -> Self {
        Self {
            hart: HartCfg::default(),
            bus: BusCfg::default(),
            harts_num: DEFAULT_HARTS_NUM,
            xtal_frequency_hz: DEFAULT_XTAL_FREQ_HZ,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LoggingCfg {
    pub level: String,
    pub style: String,
}

impl Default for LoggingCfg {
    fn default() -> Self {
        Self {
            level: DEFAULT_LOG_LEVEL.to_string(),
            style: DEFAULT_LOG_STYLE.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EmulatorCfg {
    pub input_file: Option<PathBuf>,
    throttle_frequency: u64,
    pub logging: LoggingCfg,
}

impl Default for EmulatorCfg {
    fn default() -> Self {
        Self {
            input_file: None,
            throttle_frequency: DEFAULT_THROTTLE_FREQ,
            logging: Default::default(),
        }
    }
}
