pub mod constants;

use serde::{Deserialize, Serialize};

use super::constants::arch::XLen;

use self::constants::*;

use crate::constants::arch::{BusAddrType, IRQ_Line};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct HartCfg {
    pub(super) pc_reset_value: XLen,

    extension_I: bool,
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
}

impl Default for MemoryCfg {
    fn default() -> Self {
        Self {
            base_addr: DEFAULT_RAM_BASE,
            size_bytes: DEFAULT_MEMORY_SIZE,
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

impl SocCfg {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors_list = Vec::new();

        if self.bus.memory.size_bytes == 0 {
            errors_list.push("memory size should be > 0".to_string());
        }
        if self.harts_num < 1 {
            errors_list.push("number of harts should be >= 1".to_string());
        }
        if !self.hart.extension_I {
            errors_list.push(
                "RISC-V base integer extension is not enabled".to_string(),
            );
        }
        for dev in &self.bus.devices {
            if dev.size_bytes < size_of::<XLen>() {
                let dev_name = &dev.name;
                errors_list.push(format!(
                    "{dev_name} device space size should be >= size of XLen"
                ));
            }
            if let Some(irq_line) = &dev.irq_line {
                if irq_line.0 == 0 {
                    let dev_name = &dev.name;
                    errors_list.push(format!(
                        "{dev_name}: IRQ line shouldn't be == 0"
                    ));
                }
            }
        }
        if self.xtal_frequency_hz == 0 {
            errors_list.push("XTAL frequency should be > 0 Hz".to_string());
        }

        if errors_list.is_empty() {
            Ok(())
        } else {
            Err(errors_list)
        }
    }
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
