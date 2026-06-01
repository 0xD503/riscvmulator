pub mod core;
pub mod extensions;
pub mod privileged;

use self::core::Context;
use self::core::registers::RegisterInterface;

use crate::constants::arch::{BusAddrType, XLen};
use crate::soc::memory_bus::SystemBus;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct RISC_V_CPU {
    pub(crate) context: Context,
}

impl RISC_V_CPU {
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
        self.context
            .executor
            .execute(&self.context.decoder, &self.context.arch_state);
    }

    pub(in crate::soc) fn memory_operation(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context
            .executor
            .memory_operation(bus, &self.context.decoder);
    }
}
