use clap::Parser;

use crate::utils;

#[allow(non_camel_case_types)]
pub struct CLI_Args {
    pub input_file: String,
    pub config_file: String,
}

pub fn parse_args() -> CLI_Args {
    let args = Args::parse();

    if args.license {
        utils::print_license_text();
        std::process::exit(0);
    }

    CLI_Args {
        input_file: args.input_file,
        config_file: args.config_file,
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, required = false, action = clap::ArgAction::SetTrue)]
    license: bool,
    #[arg(long, required = true)]
    input_file: String,
    #[arg(long, required = true)]
    config_file: String,
}
