use std::sync::Arc;

use crate::BusAddrType;
use riscvmulator_soc::globals::exit_codes::ExitCode;
use riscvmulator_soc::soc::memory_bus::SystemBus;

pub type PeriodicTask = Box<dyn FnMut(&PeriodicTaskArgs) -> HookAction + Send>;

pub struct PeriodicTaskArgs {
    pub bus: Arc<SystemBus<BusAddrType>>,
}

pub enum HookAction {
    None,
    // Halt,
    Stop(ExitCode),
}

impl PeriodicTaskArgs {
    pub fn new(bus: Arc<SystemBus<BusAddrType>>) -> Self {
        PeriodicTaskArgs { bus }
    }
}
