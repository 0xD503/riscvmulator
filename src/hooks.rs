use std::sync::Arc;

use crate::{
    BusAddrType, globals::exit_codes::ExitCode, soc::memory_bus::SystemBus,
};

pub(crate) type PeriodicalTask =
    Box<dyn FnMut(&PeriodicalTaskArgs) -> HookAction + Send>;

pub struct PeriodicalTaskArgs {
    pub bus: Arc<SystemBus<BusAddrType>>,
    // addr: BusAddrType,
}

pub enum HookAction {
    None,
    // Halt,
    Stop(ExitCode),
}

impl PeriodicalTaskArgs {
    pub fn new(
        bus: Arc<SystemBus<BusAddrType>>, /*, addr: BusAddrType*/
    ) -> Self {
        PeriodicalTaskArgs { bus, /*, addr*/ }
    }
}
