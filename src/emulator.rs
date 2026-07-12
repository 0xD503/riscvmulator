mod constants;
mod elf_loader;

use std::fmt::Display;
use std::sync::atomic::{self, AtomicBool};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Barrier, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{error::Error, fmt::Formatter};

use log::info;

use crate::emulator::constants::{EmulatorEvent, PERIODICAL_TASKS_PERIOD_MS};
use crate::globals::exit_codes::ExitCode;
use crate::hooks::{PeriodicalTask, PeriodicalTaskArgs};
use crate::soc::hart::Hart;
use crate::{config::Config, soc::SoC};

use self::elf_loader::{ElfData, ElfError, load_elf};

pub struct Emulator {
    soc: SoC,

    // control loop stuff
    hart_join_handles: Vec<JoinHandle<Hart>>,
    start_trigger: Arc<Barrier>,

    halted: Arc<AtomicBool>,
    in_interactive_mode: bool,
    run_loop_stopped: Arc<AtomicBool>, // run_state: RunState,

    emulator_event_sender: Sender<EmulatorEvent>,
    emulator_event_receiver: Receiver<EmulatorEvent>,

    periodical_tasks: Arc<Mutex<Vec<PeriodicalTask>>>, //
    periodical_tasks_join_handles: Vec<JoinHandle<i32>>,
}

impl Emulator {
    pub fn build_from_config(
        config: &Config,
    ) -> Result<Emulator, EmulatorError> {
        let buildable_soc: SoC = SoC::new(&config.soc);

        info!("SoC was created");

        let (sender, receiver) = mpsc::channel::<EmulatorEvent>();

        Ok(Emulator {
            soc: buildable_soc,

            hart_join_handles: Vec::with_capacity(
                config.soc.harts_num as usize,
            ),
            // +1 is for triggering thread
            start_trigger: Arc::new(Barrier::new(
                config.soc.harts_num as usize + 1,
            )),

            halted: Arc::new(AtomicBool::new(true)),
            in_interactive_mode: true,
            run_loop_stopped: Arc::new(AtomicBool::new(false)),

            emulator_event_sender: sender,
            emulator_event_receiver: receiver,

            periodical_tasks: Default::default(),
            periodical_tasks_join_handles: Vec::with_capacity(10),
        })
    }

    pub fn load_elf(&mut self, path: String) -> Result<ElfData, ElfError> {
        let load_result = load_elf(path.into(), &mut self.soc);
        if load_result.is_ok() {
            self.in_interactive_mode = false;
            self.halted.store(false, atomic::Ordering::Relaxed);
            // self.elf_loaded = true;
        }

        load_result
    }

    pub fn add_periodical_task(&mut self, task: PeriodicalTask) {
        self.periodical_tasks.lock().unwrap().push(task);
    }

    fn spawn_hart_threads(&mut self) {
        // `drain(..)` returns iterator to all items of vector and passes
        // ownership for each item on iteration
        for mut hart in self.soc.harts.drain(..) {
            let start_trigger = Arc::clone(&self.start_trigger);
            let bus = Arc::clone(&self.soc.bus);
            let halted = Arc::clone(&self.halted);
            // thread::Builder::name(self, "core {i}")
            self.hart_join_handles.push(thread::spawn(move || {
                start_trigger.wait();

                while !halted.load(atomic::Ordering::Relaxed) {
                    hart.step(&bus);
                }
                // self.run();

                // return hart ownership back to original owner (SoC)
                hart
            }));
        }
    }

    fn spawn_hooks_executors(&mut self) {
        let tasks = Arc::clone(&self.periodical_tasks);
        let halted = Arc::clone(&self.halted);
        let run_loop_stopped = Arc::clone(&self.run_loop_stopped);
        let args =
            PeriodicalTaskArgs::new(Arc::clone(&self.soc.bus) /*, 0 */);
        self.periodical_tasks_join_handles
            .push(thread::spawn(move || {
                let mut exit_status = Some(ExitCode::Success as i32);
                for task in tasks.lock().unwrap().iter_mut() {
                    // let args = PeriodicalTaskArgs {};
                    exit_status = task(&args);
                }

                thread::sleep(Duration::from_millis(
                    PERIODICAL_TASKS_PERIOD_MS,
                ));

                exit_status.unwrap()
            }));
    }

    fn wait_until_start_trigger(&self) {
        self.start_trigger.wait();
    }

    fn trigger_start(&self) {
        self.wait_until_start_trigger();
    }

    pub fn run(&mut self) {
        self.run_loop();
    }

    fn run_loop(&mut self) {
        let mut started = false;

        loop {
            if self.in_interactive_mode {
                self.interactive_mode();
            } else if !self.halted.load(atomic::Ordering::Relaxed) {
                if !started {
                    // it doesn't run actually, it takes ownership of hart
                    // threads from SoC, puts them waitin on a barrier until
                    // real run trigger
                    self.spawn_hart_threads();

                    // start trigger
                    self.trigger_start();

                    // start periodical tasks and hooks executor thread
                    self.spawn_hooks_executors();

                    started = true;
                }
            } else if self.halted.load(atomic::Ordering::Relaxed) {
                if self.run_loop_stopped.load(atomic::Ordering::Relaxed) {
                    // info!("Emulator stopped");
                    break;
                }
                todo!("handle halted state");
            }
        }

        // give ownership of harts back to SoC
        self.soc.harts = self
            .hart_join_handles
            .drain(..)
            .map(|hart_thread| hart_thread.join().unwrap())
            .collect();
        // join tasks/hook threads
        self.periodical_tasks_join_handles
            .drain(..)
            .for_each(|thread| {
                thread.join().unwrap();
            });

        // info!("Emulator stopped");
    }

    fn interactive_mode(&self) {
        info!("Emulator is running in interactive mode");
        todo!("interactive mode");
        //
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
