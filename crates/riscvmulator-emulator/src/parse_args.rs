use clap::Parser;
use serde::Serialize;

use crate::utils;

use riscvmulator_soc::globals::exit_codes::ExitCode;

#[allow(non_camel_case_types)]
#[derive(Parser, Serialize, Clone, Debug)]
#[command(version, about)]
pub struct CLI_Args {
    #[arg(long, required = false, action = clap::ArgAction::SetTrue)]
    #[serde(skip)] // not a config value
    license: bool,
    #[arg(long, required = false)]
    #[serde(skip)] // not a config value, but path to config
    pub(crate) config_file: Option<String>,

    #[arg(long, required = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_file: Option<String>,
    #[arg(long, required = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) harts_num: Option<u32>,
    #[arg(long, required = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) memory_size: Option<usize>,
    #[arg(long, required = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) throttle_frequency: Option<u64>,
    #[arg(long, required = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) log_level: Option<String>,
}

pub fn parse_args() -> CLI_Args {
    let args = CLI_Args::parse();

    if args.license {
        utils::print_license_text();
        std::process::exit(ExitCode::Success as i32);
    }

    args
}
