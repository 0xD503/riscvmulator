use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct HartCfg {
    // #[serde(default = "true")]
    extension_I: bool,
    extension_M: Option<bool>,
    extension_A: Option<bool>,
    extension_F: Option<bool>,
    extension_D: Option<bool>,
    extension_C: Option<bool>,
    extension_Zifencei: Option<bool>,
    extension_Zicsr: Option<bool>,
}

#[derive(Deserialize, Default, Debug)]
struct MemoryCfg {
    size_bytes: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
pub struct SocCfg {
    hart: Option<HartCfg>,
    harts_num: u32,
    memory: Option<MemoryCfg>,
}

#[derive(Deserialize, Debug)]
struct EmulatorCfg {
    log_level: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub soc: Option<SocCfg>,
    emulator: Option<EmulatorCfg>,
}

pub fn get_config(config_path: String) -> Config {
    let binding = fs::read_to_string(config_path).unwrap();
    let file_content = binding.as_str();
    let config: Config = toml::from_str(file_content).unwrap();

    config
    // let mem_size = config
    //     .soc
    //     .unwrap_or_default()
    //     .memory
    //     .unwrap_or_default()
    //     .size_bytes
    //     .unwrap_or(262144);
    // dbg!("{mem_size:?}");
}
