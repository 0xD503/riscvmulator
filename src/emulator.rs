mod constants;
pub(crate) mod elf_loader;

use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::SendError;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Barrier, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{error::Error, fmt::Formatter};

use log::error;
use log::info;

use crate::BusAddrType;
use crate::Device;
use crate::IODeviceError;
use crate::config::DeviceCfg;
use crate::emulator::constants::{EmulatorEvent, PERIODICAL_TASKS_PERIOD_MS};
use crate::globals::exit_codes::ExitCode;
use crate::hooks::{HookAction, PeriodicalTask, PeriodicalTaskArgs};
use crate::soc::hart::Hart;
// use crate::soc::peripherals::clint::CLINT;
use crate::soc::peripherals::common::constants::{
    DEV_KIND_CLINT, DEV_KIND_PLIC, DEV_KIND_UART,
};
// use crate::soc::peripherals::plic::PLIC;
use crate::soc::peripherals::uart::UART;
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

    emulator_event_sender: Arc<Sender<EmulatorEvent>>,
    emulator_event_receiver: Receiver<EmulatorEvent>,

    periodical_tasks: Arc<Mutex<Vec<PeriodicalTask>>>, //
    periodical_tasks_join_handles: Vec<JoinHandle<()>>,

    input_file: Option<String>,

    devices_available_for_use: HashMap<
        String, /* device_kind */
        fn(BusAddrType, usize, String, String) -> Box<dyn Device>, /* device_creator */
    >,
}

impl Emulator {
    pub fn build_from_config(
        config: Config,
    ) -> Result<Emulator, EmulatorError> {
        let buildable_soc: SoC = SoC::new(&config.soc);

        // info!("SoC was created");

        let (sender, receiver) = mpsc::channel::<EmulatorEvent>();

        let mut emulator = Emulator {
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

            emulator_event_sender: Arc::new(sender),
            emulator_event_receiver: receiver,

            periodical_tasks: Default::default(),
            periodical_tasks_join_handles: Vec::with_capacity(16),

            input_file: config.input_file(),

            devices_available_for_use: HashMap::with_capacity(16),
        };
        emulator.devices_fill_available();
        emulator.devices_create_requested(config.soc.bus.devices);

        Ok(emulator)
    }

    pub fn loop_prepare_non_interactive(
        &mut self,
    ) -> Result<(), SendError<EmulatorEvent>> {
        self.emulator_event_sender.send(EmulatorEvent::Start)
    }

    pub fn loop_prepare_interactive(
        &mut self,
    ) -> Result<(), SendError<EmulatorEvent>> {
        self.emulator_event_sender
            .send(EmulatorEvent::InteractiveMode)
    }

    pub fn run(&mut self) -> Result<(), ExitCode> {
        let exit_code = self.loop_run();
        self.loop_tear_down();

        exit_code
    }

    pub fn add_periodical_task(&mut self, task: PeriodicalTask) {
        self.periodical_tasks.lock().unwrap().push(task);
    }

    pub fn load_elf(&mut self) -> Result<ElfData, ElfError> {
        let Some(path) = self.input_file.as_deref() else {
            return Err(ElfError::NoInputFileProvided);
        };
        let load_result = load_elf(Path::new(path), &mut self.soc);
        if load_result.is_ok() {
            self.in_interactive_mode = false;
            self.halted.store(false, Ordering::Relaxed);
            // self.elf_loaded = true;
        }

        load_result
    }

    pub fn input_file(&self) -> Option<String> {
        self.input_file.clone()
    }

    fn devices_fill_available(&mut self) {
        self.devices_available_for_use
            .insert(DEV_KIND_UART.to_string(), UART::create_dev);
        // self.devices_available_for_use
        //     .insert(DEV_KIND_CLINT.to_string(), CLINT::create_dev);
        // self.devices_available_for_use
        //     .insert(DEV_KIND_PLIC.to_string(), PLIC::create_dev);
    }

    fn devices_create_requested(
        &mut self,
        devices: Vec<DeviceCfg>,
    ) -> Result<(), IODeviceError> {
        let bus = Arc::clone(&self.soc.bus);
        for device in devices {
            let dev_kind = &device.kind;
            if let Some(device_creator) =
                self.devices_available_for_use.get(dev_kind)
            {
                let device_handle = device_creator(
                    device.base_addr,
                    device.size_bytes,
                    device.kind,
                    device.name,
                );
                bus.devices.attach_device(device_handle)?;
                // if !device_created {
                //     error!("failed to create device");
                //     // Err(IODeviceError::Create(dev_kind))
                //     todo!();
                // }
            } else {
                error!(
                    "unknown device kind: {dev_kind}, skipping it's creation"
                );
                // Err(IODeviceError::Create(dev_kind))
            }
        }

        dbg!("GREPME: {}", &bus.devices.tree);
        Ok(())
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

                while !halted.load(Ordering::Relaxed) {
                    hart.step(&bus);
                }

                // return hart ownership back to original owner (SoC)
                hart
            }));
        }
    }

    fn spawn_hooks_executors(&mut self) {
        let tasks = Arc::clone(&self.periodical_tasks);
        let halted = Arc::clone(&self.halted);
        let run_loop_stopped = Arc::clone(&self.run_loop_stopped);
        let args = PeriodicalTaskArgs::new(Arc::clone(&self.soc.bus));
        let event_sender = Arc::clone(&self.emulator_event_sender);

        self.periodical_tasks_join_handles.push(thread::spawn(move || {
            let mut need_to_stop_hooks = false;

            // TODO: `halted` or `run_loop_stopped`???
            while !halted.load(Ordering::Relaxed) {
                for task in tasks.lock().unwrap().iter_mut() {
                    let hook_action = task(&args);

                    match hook_action {
                        HookAction::Stop(status_code) => {
                            if let Err(err) = event_sender
                                .send(EmulatorEvent::Stop(status_code))
                            {
                                error!("event receiver died before senders extinction: {err}");
                                todo!("decide what to do or make sure it won't happen");
                            }
                            need_to_stop_hooks = true;
                        }

                        HookAction::None => continue,
                    }
                }

                if need_to_stop_hooks {
                    break;
                }

                thread::sleep(Duration::from_millis(
                    PERIODICAL_TASKS_PERIOD_MS,
                ));
            }
        }));
    }

    fn wait_until_start_trigger(&self) {
        self.start_trigger.wait();
    }

    fn trigger_start(&self) {
        self.wait_until_start_trigger();
    }

    fn loop_run(&mut self) -> Result<(), ExitCode> {
        let exit_code;

        loop {
            let event = self.emulator_event_receiver.recv();
            if let Ok(received_event) = event {
                match received_event {
                    EmulatorEvent::Start => {
                        self.loop_start();
                    }
                    EmulatorEvent::Halt => {
                        self.loop_halt();
                    }
                    EmulatorEvent::Stop(status_code) => {
                        self.loop_stop();
                        exit_code = match status_code {
                            ExitCode::Success => Ok(()),

                            _ => Err(status_code),
                        };
                        break;
                    }
                    EmulatorEvent::InteractiveMode => {
                        self.interactive_mode();
                    }
                }
            } else {
                error!("Event receiver stopped: {}", event.unwrap_err());
                exit_code = Err(ExitCode::Error);
                break;
            }
        }

        info!("Emulator stopped");

        exit_code
    }

    fn loop_start(&mut self) {
        // it doesn't run actually, it takes ownership of hart
        // threads from SoC, puts them waitin on a barrier until
        // real run trigger
        self.spawn_hart_threads();

        // start periodical tasks and hooks executor thread
        self.spawn_hooks_executors();

        // start trigger
        self.trigger_start();
    }

    fn loop_halt(&mut self) {
        self.halted.store(true, Ordering::Relaxed);
        todo!();
    }

    fn loop_stop(&mut self) {
        self.halted.store(true, Ordering::Relaxed);
        self.run_loop_stopped.store(true, Ordering::Relaxed);
    }

    fn interactive_mode(&self) {
        info!("Emulator is running in interactive mode");
        todo!("interactive mode");
        //
    }

    fn loop_tear_down(&mut self) {
        // give ownership of harts back to SoC
        self.soc.harts = self
            .hart_join_handles
            .drain(..)
            .map(|hart_thread| {
                hart_thread.join().unwrap_or_else(|err| {
                    todo!("Is it even possible???: {err:#?}");
                })
            })
            .collect();
        // join tasks/hook threads
        self.periodical_tasks_join_handles
            .drain(..)
            // .iter_mut()
            .for_each(|thread| {
                thread.join().unwrap_or_else(|err| {
                    todo!("Is it even possible???: {err:#?}");
                });
            });
    }

    // fn on_halted(&self) {
    //     todo!("unimplemented halted state handling");
    // }
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
