use crate::config::{
    Config, ConfigError,
    constants::{DEFAULT_LOG_LEVEL, DEFAULT_LOG_STYLE},
};

pub fn init_logger(config: &Result<Config, ConfigError>) {
    let default_log_level = DEFAULT_LOG_LEVEL.to_string();
    let default_log_style = DEFAULT_LOG_STYLE.to_string();
    // let log_level = &config.unwrap_or(&default_log_level).emulator.log_level;
    let (log_level, log_style);
    if let Ok(conf) = config {
        log_level = &conf.emulator.logging.level;
        log_style = &conf.emulator.logging.style;
    } else {
        log_level = &default_log_level;
        log_style = &default_log_style;
    }
    // set default log level (unless specified with RVMULATOR_EMULATOR__LOGGING__LEVEL environment variable)
    // env_logger::Builder::from_env(
    //     env_logger::Env::default().default_filter_or(log_level),
    // )
    env_logger::Builder::new()
        .parse_filters(log_level)
        .parse_write_style(log_style)
        .init();
}
