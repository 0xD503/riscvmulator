pub mod constants;

use std::{
    error::Error,
    fmt::{self, Formatter},
    fs,
};

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

// use super::CLI_Args;
use super::constants::arch::XLen;

use self::constants::*;

use crate::{
    constants::arch::{BusAddrType, IRQ_Line},
    globals::exit_codes::ExitCode,
    parse_args::CLI_Args,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub(super) struct MemoryCfg {
    pub(super) base_addr: BusAddrType,
    pub(super) size_bytes: usize,
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
    irq_line: Option<IRQ_Line>,
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
    pub(super) memory: MemoryCfg,
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
    pub(super) hart: HartCfg,
    pub bus: BusCfg,
    pub harts_num: u32,
    xtal_frequency_hz: u64,
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
    pub input_file: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
    pub fn build(cli_args: &CLI_Args) -> Result<Config, ConfigError> {
        // debug!("Building emulator config");

        let mut config = Figment::from(Serialized::defaults(Config::default()));
        if let Some(conf_file) = &cli_args.config_file {
            let file_content = fs::read_to_string(conf_file)
                .map_err(|_| ConfigError::Read(Some(conf_file.clone())))?;
            config = config.merge(Toml::string(&file_content));
        } else {
            // info!("Configuration file was not provided");
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

        if let Some(throttle_frequency) = &cli_args.throttle_frequency {
            config = config.merge(Serialized::default(
                "emulator.throttle_frequency",
                throttle_frequency,
            ));
        }
        if let Some(log_level) = &cli_args.log_level {
            config = config.merge(Serialized::default(
                "emulator.logging.level",
                log_level,
            ));
        }
        if let Some(input_file) = &cli_args.input_file {
            config = config
                .merge(Serialized::default("emulator.input_file", input_file));
        }

        let mut config: Config =
            config.extract().map_err(ConfigError::Build)?;

        config.validate()?;
        config.resolve();

        Ok(config)
    }

    pub fn emulator(&self) -> &EmulatorCfg {
        &self.emulator
    }

    pub fn soc(&self) -> &SocCfg {
        &self.soc
    }

    fn validate(&self) -> Result<(), ConfigError> {
        let mut errors_list = Vec::new();

        if self.soc.bus.memory.size_bytes == 0 {
            errors_list.push("memory size should be > 0".to_string());
        }
        if self.soc.harts_num < 1 {
            errors_list.push("number of harts should be >= 1".to_string());
        }
        if !self.soc.hart.extension_I {
            errors_list.push(
                "RISC-V base integer extension is not enabled".to_string(),
            );
        }
        for dev in &self.soc.bus.devices {
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
        if self.soc.xtal_frequency_hz == 0 {
            errors_list.push("XTAL frequency should be > 0 Hz".to_string());
        }

        if self.emulator.logging.level != "off"
            && self.emulator.logging.level != "error"
            && self.emulator.logging.level != "warn"
            && self.emulator.logging.level != "info"
            && self.emulator.logging.level != "debug"
            && self.emulator.logging.level != "trace"
        {
            let level = &self.emulator.logging.level;
            errors_list.push(format!("{level}: invalid log level"));
        }
        if self.emulator.logging.style != "auto"
            && self.emulator.logging.style != "always"
            && self.emulator.logging.style != "never"
        {
            let style = &self.emulator.logging.style;
            errors_list.push(format!("{style}: invalid log style"));
        }

        if errors_list.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::Validation(errors_list))
        }
    }

    /// autoresolves some values from existing ones
    fn resolve(&mut self) {
        // TODO: autoresolves
    }

    pub fn input_file(&self) -> Option<String> {
        self.emulator.input_file.clone()
    }

    pub fn set_input_elf_path(&mut self, path: String) {
        self.emulator.input_file = Some(path);
    }

    // pub fn set_config_file_path(&mut self, path: String) {
    //     self.emulator.config_file_path = Some(path);
    // }
}

#[derive(Debug)]
pub enum ConfigError {
    Read(Option<String>),
    Build(figment::Error),
    Validation(Vec<String>),
}

impl ConfigError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Read(_) => ExitCode::ConfigRead as i32,
            Self::Build(_) => ExitCode::ConfigBuild as i32,
            Self::Validation(_) => ExitCode::ConfigValidation as i32,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Read(path) => {
                if let Some(path) = path {
                    writeln!(formatter, "failed to open file: {path}")
                } else {
                    writeln!(formatter, "failed to open file: path not set")
                }
            }
            Self::Build(error) => {
                writeln!(formatter, "failed to build config: {error}")
            }
            Self::Validation(errors) => {
                writeln!(
                    formatter,
                    "The following errors occurred during config validation:"
                )?;
                for err in errors {
                    writeln!(formatter, "{err}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for ConfigError {}
