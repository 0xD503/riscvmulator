use std::fmt::Display;
use std::{error::Error, fmt::Formatter};

use log::info;

use crate::elf_loader::{ElfData, ElfError};
use crate::globals::exit_codes::ExitCode;
use crate::{config::Config, elf_loader::load_elf, soc::SoC};

pub struct Emulator {
    soc: SoC,
    // config: Config,
    halted: bool,
    // elf_loaded: bool,
    in_interactive_mode: bool,
    // run_state: RunState,
}

impl Emulator {
    pub fn build_from_config(
        config: &Config,
    ) -> Result<Emulator, EmulatorError> {
        let mut buildable_soc: SoC = SoC::new(&config.soc);
        buildable_soc.reset(&config.soc);
        info!("SoC was reset");

        Ok(Emulator {
            soc: buildable_soc,
            // config: config,
            halted: true,
            // elf_loaded: false,
            in_interactive_mode: true,
        })
    }

    pub fn load_elf(&mut self, path: String) -> Result<ElfData, ElfError> {
        let load_result = load_elf(path.into(), &mut self.soc);
        if load_result.is_ok() {
            self.in_interactive_mode = false;
            self.halted = false;
            // self.elf_loaded = true;
        }

        load_result
    }

    pub fn run(&mut self) {
        loop {
            if self.in_interactive_mode {
                self.interactive_mode();
            } else if !self.halted {
                // self.halted = false;
                self.run_loop();
            } else {
                break;
            }
        }
    }

    fn interactive_mode(&self) {
        info!("CPU is running in interactive mode");
        todo!("interactive mode");
        //
    }

    fn run_loop(&mut self) {
        while !self.halted {
            self.soc.step();
        }
    }

    // fn on_halted(&self) {
    //     todo!("unimplemented halted state handling");
    // }

    // TODO: remove testing facilities
    pub fn ready_for_testing(&mut self) {
        self.soc.ready_for_testing();
    }
}

#[derive(Debug)]
pub enum EmulatorError {
    // Config(i32),
}

// enum RunState {
//     Interactive,
//     Running,
//     Halted,
// }

impl EmulatorError {
    pub fn exit_code(&self) -> i32 {
        match self {
            // EmulatorError::Config(exit_code) => *exit_code,
            //
            _ => ExitCode::Error as i32,
        }
    }
}

impl Display for EmulatorError {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            // EmulatorError::Config(exit_code) => {
            //     write!(
            //         formatter,
            //         concat!(
            //             "failed to configure emulator, terminating ",
            //             "with exit code {exit_code}"
            //         ),
            //         exit_code = exit_code
            //     )
            // }
            //
            _ => {
                write!(formatter, "unknown error")
            }
        }
    }
}

impl Error for EmulatorError {}
