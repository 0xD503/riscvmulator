pub mod hart;
pub mod memory;
pub mod memory_bus;

mod peripherals;

use std::sync::Barrier;
use std::thread;

use crate::config::SocCfg;

use super::constants::arch::BusAddrType;
use super::constants::soc::MEM_RESET_VALUE;

use self::hart::Hart;
use self::memory_bus::{Bus, SystemBus};

pub struct SoC {
    pub(crate) harts: Vec<Hart>,
    // memory and memory mapped IO
    // von Neumann architechture, so common bus is used for instructions/data
    pub(crate) bus: SystemBus<BusAddrType>,

    start_trigger: Barrier,
}

impl SoC {
    pub fn new(cfg: &SocCfg) -> Self {
        let mut soc = Self::get_reset_instance(cfg);

        // Power-on reset
        soc.reset(cfg);

        // it doesn't run actually, just puts them waitin on a barrier until
        // real run trigger
        // soc

        soc
    }

    pub fn reset(&mut self, cfg: &SocCfg) {
        for (hard_id, hart) in self.harts.iter_mut().enumerate() {
            hart.reset(cfg, hard_id);
        }
        self.bus.reset();

        // // fill memory with NOP's
        // self.bus.fill_with_nops();
    }

    fn get_reset_instance(cfg: &SocCfg) -> Self {
        Self {
            harts: vec![Hart::new(cfg); cfg.harts_num as usize],
            bus: SystemBus::new(
                MEM_RESET_VALUE,
                cfg.bus.memory.base_addr,
                cfg.bus.memory.size_bytes,
            ),
            // +1 is for triggering thread
            start_trigger: Barrier::new(cfg.harts_num as usize + 1),
        }
    }

    pub fn step(&mut self) {
        // TODO should be multithreaded
        for hart in self.harts.iter_mut() {
            hart.step(&mut self.bus);
        }
    }

    fn run(&mut self) {
        self.start_trigger.wait();

        loop {
            self.step();
        }
    }

    fn spawn_core_threads(&mut self) {
        // thread::Builder::name(self, "core {i}")
        let bus = &self.bus;
        let start_trigger = &self.start_trigger;
        thread::scope(|scope| {
            for hart in self.harts.iter_mut() {
                scope.spawn(|| {
                    // on spawn wait signal to start actual execution cycle
                    start_trigger.wait();

                    loop {
                        hart.step(bus);
                    }
                });
                // s.spawn(self.run());
            }
        });
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
