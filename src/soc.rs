pub mod cpu;
pub mod memory;
pub mod memory_bus;
mod peripherals;

use crate::config::SocCfg;

use self::cpu::Hart;
use self::memory_bus::{Bus, SystemBus};

use super::constants::arch::BusAddrType;
use super::constants::soc::MEM_RESET_VALUE;

pub struct SoC {
    pub(crate) harts: Vec<Hart>,
    // memory and memory mapped IO
    // von Neumann architechture, so common bus is used for instructions/data
    pub(crate) bus: SystemBus<BusAddrType>,
}

impl SoC {
    pub fn new(cfg: &SocCfg) -> Self {
        Self::get_reset_instance(cfg)
    }

    pub fn reset(&mut self, cfg: &SocCfg) {
        for hart in self.harts.iter_mut() {
            hart.reset(cfg);
        }
        self.bus.reset();

        // // fill memory with NOP's
        // self.bus.fill_with_nops();
    }

    fn get_reset_instance(cfg: &SocCfg) -> Self {
        let soc = Self {
            harts: vec![Hart::new(cfg); cfg.harts_num as usize],
            bus: SystemBus::new(
                MEM_RESET_VALUE,
                cfg.bus.memory.base_addr,
                cfg.bus.memory.size_bytes,
            ),
        };
        // TODO: set Hart regs and device state (HW bringup)
        // buildable_soc.reset(&config.soc);
        soc.reset_hw_state(cfg);

        soc
    }

    fn reset_hw_state(&self, cfg: &SocCfg) {
        //
    }

    pub fn step(&mut self) {
        // TODO should be multithreaded
        for hart in self.harts.iter_mut() {
            hart.step(&mut self.bus);
        }
    }

    // testing facilities
    // TODO: conditional compilation for test facilities
    pub fn ready_for_testing(&mut self) {
        self.bus.ready_for_testing();
    }

    // TODO: conditional compilation for test facilities
    pub fn set_htif(&mut self, addr: BusAddrType) {
        self.bus.set_htif(addr);
    }
}
