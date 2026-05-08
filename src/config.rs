use std::{env, fs};

use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct MemoryCfg {
    size_bytes: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
struct SocCfg {
    memory: Option<MemoryCfg>,
}

#[derive(Deserialize, Debug)]
struct EmulatorCfg {
    //
}

#[derive(Deserialize, Debug)]
struct Config {
    soc: Option<SocCfg>,
    emulator: Option<EmulatorCfg>,
}

pub fn configure() {
    const DEFAULT_CONFIG_PATH: &str = "cfg/emulator.toml";
    let config_path = DEFAULT_CONFIG_PATH;
    let binding = fs::read_to_string(config_path).unwrap();
    let file_content = binding.as_str();
    let config: Config = toml::from_str(file_content).unwrap();
    let mem_size = config
        .soc
        .unwrap_or_default()
        .memory
        .unwrap_or_default()
        .size_bytes
        .unwrap_or_default();
    // debug!("{mem_size:?}");
}
