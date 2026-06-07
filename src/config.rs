mod constants;

use std::str::FromStr;

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use log::error;
use serde::{Deserialize, Serialize};

use super::CLI_Args;
use super::constants::arch::XLen;

use self::constants::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub(super) struct HartCfg {
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
pub(super) struct MemoryCfg {
    pub(super) size_bytes: usize,
}

impl Default for MemoryCfg {
    fn default() -> Self {
        Self {
            size_bytes: DEFAULT_MEMORY_SIZE,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusCfg {
    pub(super) memory: MemoryCfg,
    pub(super) ram_base: XLen,
    ram_end: XLen,
}

impl Default for BusCfg {
    fn default() -> Self {
        Self {
            memory: MemoryCfg::default(),
            ram_base: DEFAULT_RAM_BASE,
            ram_end: DEFAULT_RAM_END,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocCfg {
    pub(super) hart: HartCfg,
    pub(super) bus: BusCfg,
    harts_num: u32,
}

impl Default for SocCfg {
    fn default() -> Self {
        Self {
            hart: HartCfg::default(),
            bus: BusCfg::default(),
            harts_num: DEFAULT_HARTS_NUM,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmulatorCfg {
    log_level: String,
    pub input_file: Option<String>,
    // config_file: Option<String>,
}

impl Default for EmulatorCfg {
    fn default() -> Self {
        Self {
            log_level: String::from_str(DEFAULT_LOG_LEVEL).unwrap(),
            input_file: None,
            // config_file: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub soc: SocCfg,
    pub emulator: EmulatorCfg,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            soc: Default::default(),
            emulator: Default::default(),
        }
    }
}

impl Config {
    pub fn build(cli_args: &CLI_Args) -> Config {
        let mut config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(cli_args.config_file.as_ref().unwrap().clone()))
            .merge(Env::prefixed(CONFIG_ENV_VAR_PREFIX));

        if let Some(input_file) = &cli_args.input_file {
            config = config
                .merge(Serialized::default("emulator.input_file", input_file));
        }
        // .merge(Serialized::defaults(cli_args))
        let mut config: Config = config
            .extract()
            // TODO: get rid of panic
            .expect("failed to build config");

        // dbg!(":?", config);
        // dbg!(":?", cli_args);
        Config::validate(&config);
        Config::resolve(&mut config);

        config
        // Config {
        //     ..Default::default()
        // }
    }

    fn validate(cfg: &Config) {
        let mut valid = true;

        if cfg.soc.bus.memory.size_bytes == 0 {
            valid = false;
            error!("Memory size should be > 0");
        }
        if cfg.soc.harts_num < 1 {
            valid = false;
            error!("Number of harts should be >= 1");
        }
        if cfg.emulator.input_file.is_none() {
            valid = false;
            error!("Input file should be provided");
        }
        // TODO: add more checks

        if !valid {
            std::process::exit(1);
        }
    }

    /// autoresloves some values from existing ones
    fn resolve(cfg: &mut Config) {
        cfg.soc.bus.ram_end =
            cfg.soc.bus.ram_base + cfg.soc.bus.memory.size_bytes as XLen;
    }
}
