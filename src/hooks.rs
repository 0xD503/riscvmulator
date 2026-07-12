use std::sync::Arc;

use crate::{BusAddrType, soc::memory_bus::SystemBus};

pub(crate) type PeriodicalTask =
    Box<dyn FnMut(&PeriodicalTaskArgs) -> Option<i32> + Send>;

pub struct PeriodicalTaskArgs {
    pub bus: Arc<SystemBus<BusAddrType>>,
    // addr: BusAddrType,
}

impl PeriodicalTaskArgs {
    pub fn new(
        bus: Arc<SystemBus<BusAddrType>>, /*, addr: BusAddrType*/
    ) -> Self {
        PeriodicalTaskArgs { bus, /*, addr*/ }
    }
}
