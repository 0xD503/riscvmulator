use riscvmulator_soc::globals::exit_codes::ExitCode;

use crate::config::ConfigError;

impl From<ConfigError> for ExitCode {
    fn from(config_err: ConfigError) -> Self {
        match config_err {
            ConfigError::Read(_) => Self::ConfigRead,
            ConfigError::Build(_) => Self::ConfigBuild,
            ConfigError::Validation(_) => Self::ConfigValidation,
            ConfigError::PathNotGiven => Self::ConfigPathNotGiven,
        }
    }
}
