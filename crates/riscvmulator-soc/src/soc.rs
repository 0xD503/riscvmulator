pub mod hart;
pub mod memory;
pub mod memory_bus;
pub mod peripherals;

use std::sync::Arc;

use crate::config::SocCfg;

use super::constants::arch::BusAddrType;
use super::constants::soc::MEM_RESET_VALUE;

use self::hart::Hart;
use self::memory_bus::{Bus, SystemBus};

pub struct SoC {
    harts: Vec<Hart>,
    // memory and memory mapped IO
    // von Neumann architechture, so common bus is used for instructions/data
    bus: Arc<SystemBus<BusAddrType>>,
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
        self.bus.reset();
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

    pub fn harts_mut(&mut self) -> &mut Vec<Hart> {
        &mut self.harts
    }

    pub fn bus(&self) -> &Arc<SystemBus<BusAddrType>> {
        &self.bus
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
}
