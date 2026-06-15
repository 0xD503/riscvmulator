pub mod core;
pub mod extensions;
pub mod privileged;

use log::trace;

use self::core::Context;
use self::core::registers::RegisterInterface;

use crate::config::SocCfg;
use crate::constants::arch::{BusAddrType, XLen};
use crate::soc::memory_bus::SystemBus;

#[derive(Clone)]
pub struct Hart {
    pub(crate) context: Context,
}

impl Hart {
    pub(in crate::soc) fn reset(&mut self, cfg: &SocCfg) {
        // TODO: init hart-id, DTB source
        self.context.reset(cfg);
    }

    pub(crate) fn set_pc(&mut self, value: XLen) {
        self.context.arch_state.PC.write(value);
    }

    fn fetch(&mut self, bus: &SystemBus<BusAddrType>) {
        self.context
            .fetch_unit
            .fetch(bus, &self.context.arch_state.PC);
    }

    fn decode(&mut self) {
        self.context.decoder.decode(
            &self.context.fetch_unit,
            &self.context.arch_state.gp_register_file,
        );
    }

    fn execute(&mut self) {
        self.context.executor.execute(
            &self.context.decoder,
            &self.context.arch_state,
            &self.context.fetch_unit,
        );
    }

    fn memory_operation(&mut self, bus: &mut SystemBus<BusAddrType>) {
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

    pub(in crate::soc) fn step(&mut self, bus: &mut SystemBus<BusAddrType>) {
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
