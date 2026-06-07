use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct MemoryCfg {
    size_bytes: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
struct SocCfg {
    cores_num: Option<u32>,
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

pub fn configure(config_path: String) {
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
    // dbg!("{mem_size:?}");
}
