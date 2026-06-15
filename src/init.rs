use super::config::{Config, ConfigError};
use super::logger::init_logger;
use super::parse_args::parse_args;

pub fn init() -> Result<Config, ConfigError> {
    let args = parse_args();
    let config = Config::build(&args);
    init_logger(&config);

    config
}
