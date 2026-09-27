pub mod core;
pub mod extensions;
pub mod privileged;

use log::trace;

use self::core::Context;
use self::core::registers::{RegisterFileInterface, RegisterInterface};

use self::core::decoder::constants::InstructionSize;
use crate::config::SocCfg;
use crate::constants::arch::{BusAddrType, XLen};
use crate::constants::soc::INITIAL_INSTRUCTION;
use crate::soc::hart::core::ArchState;
use crate::soc::hart::core::decoder::Decoder;
use crate::soc::hart::core::executor::Executor;
use crate::soc::hart::core::fetch_unit::FetchUnit;
use crate::soc::hart::core::gp_registers::{Register, RegisterFile};
use crate::soc::hart::extensions::zicsr::CSRFile;
use crate::soc::memory_bus::SystemBus;

#[derive(Clone)]
pub struct Hart {
    pub(crate) context: Context,
}

impl Hart {
    pub(in crate::soc) fn new(cfg: &SocCfg) -> Self {
        Self::get_reset_instance(cfg)
    }

    pub(in crate::soc) fn reset(&mut self, cfg: &SocCfg, hartid: usize) {
        // TODO: init hart-id, DTB source
        self.context.reset(cfg, hartid);
    }

    fn get_reset_instance(cfg: &SocCfg) -> Self {
        Self {
            context: Context {
                // TODO: init hart-id, DTB source
                arch_state: ArchState {
                    PC: Register::new(cfg.hart.pc_reset_value),
                    gp_register_file: RegisterFile::new(),
                    control_status_registers: CSRFile::new(),
                },
                fetch_unit: FetchUnit::new(
                    INITIAL_INSTRUCTION,
                    InstructionSize::I32,
                ),
                decoder: Decoder::new(),
                executor: Executor::new(cfg.hart.pc_reset_value),
            },
        }
    }

    pub fn set_pc(&mut self, value: XLen) {
        self.context.arch_state.PC.write(value);
    }

    fn fetch(&mut self, bus: &SystemBus<BusAddrType>) {
        self.context
            .fetch_unit
            .fetch(bus, &self.context.arch_state.PC);
    }

    fn decode(&mut self) {
        self.context
            .decoder
            .decode(&self.context.arch_state, &self.context.fetch_unit);
    }

    fn execute(&mut self) {
        self.context.executor.execute(
            &self.context.decoder,
            &self.context.arch_state,
            &self.context.fetch_unit,
        );
    }

    fn memory_operation(&mut self, bus: &SystemBus<BusAddrType>) {
        self.context
            .executor
            .memory_operation(bus, &self.context.decoder);
    }

    // fn write_back() {
    //     //
    // }

    // fn advance_pc() {
    //     //, fetch_unit: &FetchUnit
    // }

    pub fn step(&mut self, bus: &SystemBus<BusAddrType>) {
        trace!("Current PC: {:#018X}", self.context.arch_state.PC.read());

        self.fetch(bus);
        self.decode();
        self.execute();
        self.memory_operation(bus);
        self.write_back();

        // step PC
        self.advance_pc();
    }
}
