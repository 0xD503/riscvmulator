use std::sync::Arc;

use crate::BusAddrType;
use riscvmulator_soc::globals::exit_codes::ExitCode;
use riscvmulator_soc::soc::memory_bus::SystemBus;

pub(crate) type PeriodicalTask =
    Box<dyn FnMut(&PeriodicalTaskArgs) -> HookAction + Send>;

pub struct PeriodicalTaskArgs {
    pub bus: Arc<SystemBus<BusAddrType>>,
}

pub enum HookAction {
    None,
    // Halt,
    Stop(ExitCode),
}

impl PeriodicalTaskArgs {
    pub fn new(bus: Arc<SystemBus<BusAddrType>>) -> Self {
        PeriodicalTaskArgs { bus }
    }
}
