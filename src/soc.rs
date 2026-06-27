pub mod hart;
pub mod memory;
pub mod memory_bus;

mod peripherals;

use std::sync::Arc;

use crate::config::SocCfg;

use super::constants::arch::BusAddrType;
use super::constants::soc::MEM_RESET_VALUE;

use self::hart::Hart;
use self::memory_bus::{Bus, SystemBus};

pub struct SoC {
    pub(crate) harts: Vec<Hart>,
    // memory and memory mapped IO
    // von Neumann architechture, so common bus is used for instructions/data
    pub(crate) bus: Arc<SystemBus<BusAddrType>>,
}

impl SoC {
    pub fn new(cfg: &SocCfg) -> Self {
        let mut soc = Self::get_reset_instance(cfg);

        // Power-on reset
        soc.reset(cfg);

        soc
    }

    pub fn reset(&mut self, cfg: &SocCfg) {
        for (hard_id, hart) in self.harts.iter_mut().enumerate() {
            hart.reset(cfg, hard_id);
        }
        let bus = Arc::clone(&self.bus);
        bus.reset();
    }

    fn get_reset_instance(cfg: &SocCfg) -> Self {
        Self {
            harts: vec![Hart::new(cfg); cfg.harts_num as usize],
            bus: Arc::new(SystemBus::new(
                MEM_RESET_VALUE,
                cfg.bus.memory.base_addr,
                cfg.bus.memory.size_bytes,
            )),
        }
    }

    // pub fn step(&mut self) {
    //     // TODO should be multithreaded
    //     for hart in self.harts.iter_mut() {
    //         hart.step(&mut self.bus);
    //     }
    // }

    // fn run(&mut self) {
    //     // self.start_trigger.wait();

    //     loop {
    //         self.step();
    //     }
    // }

    // testing facilities
    // TODO: conditional compilation for test facilities
    pub fn ready_for_testing(&mut self) {
        let bus = Arc::clone(&self.bus);
        bus.set_ready_for_testing();
    }

    // TODO: conditional compilation for test facilities
    pub fn set_htif(&mut self, addr: BusAddrType) {
        let bus = Arc::clone(&self.bus);
        bus.set_htif(addr);
    }
}
