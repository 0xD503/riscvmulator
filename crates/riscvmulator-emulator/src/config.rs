pub mod constants;

use std::{
    error::Error,
    fmt::{self, Formatter},
    fs,
    path::PathBuf,
};

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

use riscvmulator_soc::config::SocCfg;
use riscvmulator_soc::globals::exit_codes::ExitCode;

use self::constants::*;

use crate::parse_args::CLI_Args;

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

        let mut config: Config = config
            .extract()
            .map_err(|err| ConfigError::Build(err.to_string()))?;

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

        let mut soc_validation_errors =
            self.soc.validate().err().unwrap_or_default();
        errors_list.append(&mut soc_validation_errors);

        if self.emulator.logging.level != "off"
            && self.emulator.logging.level != "error"
            && self.emulator.logging.level != "warn"
            && self.emulator.logging.level != "info"
            && self.emulator.logging.level != "debug"
            && self.emulator.logging.level != "trace"
        {
            errors_list.push(format!(
                "{}: invalid log level",
                &self.emulator.logging.level
            ));
        }
        if self.emulator.logging.style != "auto"
            && self.emulator.logging.style != "always"
            && self.emulator.logging.style != "never"
        {
            errors_list.push(format!(
                "{}: invalid log style",
                &self.emulator.logging.style
            ));
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

    pub fn input_file(&self) -> Option<PathBuf> {
        self.emulator.input_file.clone()
    }

    pub fn set_input_elf_path(&mut self, path: PathBuf) {
        self.emulator.input_file = Some(path);
    }

    // pub fn set_config_file_path(&mut self, path: String) {
    //     self.emulator.config_file_path = Some(path);
    // }
}

#[derive(Debug)]
pub enum ConfigError {
    Read(Option<String>),
    Build(String),
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
