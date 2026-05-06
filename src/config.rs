use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
struct MemoryCfg {
    size_bytes: Option<usize>,
}

#[derive(Deserialize)]
struct SocConfig {
    soc: Option<MemoryCfg>,
}

pub fn configure() {
    const DEFAULT_CONFIG_PATH: &str = "cfg/emulator.toml";
    let config_path = DEFAULT_CONFIG_PATH;
    let binding = fs::read_to_string(config_path).unwrap();
    let file_content = binding.as_str();
    let config: SocConfig = toml::from_str(file_content).unwrap();
}
