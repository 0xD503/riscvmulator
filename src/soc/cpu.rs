pub mod core;
pub mod extensions;
pub mod privileged;

use self::core::Context;
use self::core::registers::RegisterInterface;

use crate::constants::arch::{BusAddrType, XLen};
use crate::soc::memory_bus::SystemBus;

#[derive(Clone)]
pub struct Hart {
    pub(crate) context: Context,
}

impl Hart {
    pub fn reset(&mut self) {
        self.context.reset();
    }

    pub(crate) fn set_pc(&mut self, value: XLen) {
        self.context.arch_state.PC.write(value);
    }

    pub(in crate::soc) fn fetch(&mut self, bus: &SystemBus<BusAddrType>) {
        self.context
            .fetch_unit
            .fetch(bus, &self.context.arch_state.PC);
    }

    pub(in crate::soc) fn decode(&mut self) {
        self.context.decoder.decode(
            &self.context.fetch_unit,
            &self.context.arch_state.gp_register_file,
        );
    }

    pub(in crate::soc) fn execute(&mut self) {
        self.context.executor.execute(
            &self.context.decoder,
            &self.context.arch_state,
            &self.context.fetch_unit,
        );
    }

    pub(in crate::soc) fn memory_operation(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context
            .executor
            .memory_operation(bus, &self.context.decoder);
    }

    // pub(in crate::soc) fn write_back() {
    //     //
    // }

    // pub(in crate::soc) fn advance_pc() {
    //     //, fetch_unit: &FetchUnit
    // }
}
