mod constants;

use std::str::FromStr;

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use log::{debug, error};
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
    pub(super) base_addr: XLen,
    pub(super) size_bytes: usize,
    ram_end: XLen,
}

impl Default for MemoryCfg {
    fn default() -> Self {
        Self {
            base_addr: DEFAULT_RAM_BASE,
            size_bytes: DEFAULT_MEMORY_SIZE,
            ram_end: DEFAULT_RAM_END,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusCfg {
    pub(super) memory: MemoryCfg,
}

impl Default for BusCfg {
    fn default() -> Self {
        Self {
            memory: MemoryCfg::default(),
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
}

impl Default for EmulatorCfg {
    fn default() -> Self {
        Self {
            log_level: String::from_str(DEFAULT_LOG_LEVEL).unwrap(),
            input_file: None,
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
        debug!("Building emulator config");

        let mut config = Figment::from(Serialized::defaults(Config::default()));
        if let Some(conf_file) = &cli_args.config_file {
            config = config.merge(Toml::file(conf_file.clone()));
        }
        config = config.merge(
            Env::prefixed(CONFIG_ENV_VAR_PREFIX)
                // if you want to use environment variable, it should use
                // double undescores (`__`) to substitute dots `.` in
                // nesting objects.
                // For example, in order to set `soc.bus.memory.size_bytes`
                // you should use prefixed environment variable:
                // `RVMULATOR_SOC__BUS__MEMORY__SIZE_BYTES`
                .split("__"),
        );
        // overrides provided by CLI args
        if let Some(harts_num) = &cli_args.harts_num {
            config =
                config.merge(Serialized::default("soc.harts_num", harts_num));
        }
        if let Some(memory_size) = &cli_args.memory_size {
            config = config.merge(Serialized::default(
                "soc.bus.memory.size_bytes",
                memory_size,
            ));
        }

        if let Some(throttle_freq) = &cli_args.throttle_freq {
            config = config.merge(Serialized::default(
                "emulator.throttle_freq",
                throttle_freq,
            ));
        }
        if let Some(log_level) = &cli_args.log_level {
            config = config
                .merge(Serialized::default("emulator.log_level", log_level));
        }
        if let Some(input_file) = &cli_args.input_file {
            config = config
                .merge(Serialized::default("emulator.input_file", input_file));
        }

        let mut config: Config = config
            .extract()
            // TODO: get rid of panic
            .expect("failed to build config");

        Config::validate(&config);
        Config::resolve(&mut config);

        config
    }

    fn validate(cfg: &Config) {
        let mut valid = true;

        if cfg.soc.bus.memory.size_bytes == 0 {
            valid = false;
            error!("memory size should be > 0");
        }
        if cfg.soc.harts_num < 1 {
            valid = false;
            error!("number of harts should be >= 1");
        }
        if cfg.soc.hart.extension_I == false {
            valid = false;
            error!("RISC-V base integer extension is not enabled");
        }
        // TODO: add more checks

        if !valid {
            std::process::exit(1);
        }
    }

    /// autoresloves some values from existing ones
    fn resolve(cfg: &mut Config) {
        cfg.soc.bus.memory.ram_end = cfg.soc.bus.memory.base_addr
            + cfg.soc.bus.memory.size_bytes as XLen;
    }
}
