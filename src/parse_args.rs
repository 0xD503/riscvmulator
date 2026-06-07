use clap::Parser;
use serde::Serialize;

use crate::utils;

#[allow(non_camel_case_types)]
#[derive(Serialize, Clone, Debug)]
pub struct CLI_Args {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) config_file: Option<String>,
}

pub fn parse_args() -> CLI_Args {
    let args = Args::parse();

    if args.license {
        utils::print_license_text();
        std::process::exit(0);
    }

    CLI_Args {
        input_file: Some(args.input_file),
        config_file: Some(args.config_file),
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, required = false, action = clap::ArgAction::SetTrue)]
    license: bool,
    #[arg(long, required = true)]
    input_file: String,
    #[arg(long, required = false)]
    config_file: String,
}
