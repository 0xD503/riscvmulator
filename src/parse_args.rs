use clap::Parser;

use crate::utils;

pub fn parse_args() {
    // TODO: add `--license` argument to print license
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    if args.license {
        utils::print_license_text();
        std::process::exit(0);
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, required = false, action = clap::ArgAction::SetTrue)]
    license: bool,
}
