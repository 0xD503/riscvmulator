pub mod cpu;
pub mod memory;
pub mod memory_bus;

use crate::config::SocCfg;

use self::cpu::Hart;
use self::cpu::core::decoder::{Decoder, constants::InstructionSize};
use self::cpu::core::executor::Executor;
use self::cpu::core::fetch_unit::FetchUnit;
use self::cpu::core::gp_registers::{Register, RegisterFile};
use self::cpu::core::registers::RegisterFileInterface;
use self::cpu::core::{ArchState, Context};
use self::cpu::extensions::zicsr::CSRFile;
use self::cpu::extensions::zicsr::ExecutorZicsrContext;
use self::memory::MemoryValueType;
use self::memory_bus::{Bus, SystemBus};

use super::constants::arch::{BusAddrType, XLen};
use super::constants::soc::{INITIAL_INSTRUCTION, MEM_RESET_VALUE};

pub struct SoC {
    pub(crate) harts: Vec<Hart>,
    // memory and memory mapped IO
    // von Neumann architechture, so common bus is used for instructions/data
    pub(crate) bus: SystemBus<BusAddrType>,
}

impl SoC {
    pub fn new(cfg: &SocCfg) -> Self {
        Self {
            harts: vec![
                Hart {
                    context: Context {
                        // TODO: init hart-id, DTB source
                        arch_state: ArchState {
                            PC: Register::new(cfg.hart.pc_reset_value),
                            gp_register_file: RegisterFile::new(),
                            control_status_registers: CSRFile::new(),
                        },
                        fetch_unit: FetchUnit::new(
                            INITIAL_INSTRUCTION,
                            InstructionSize::I32,
                        ),
                        decoder: Decoder {
                            ..Default::default()
                        },
                        executor: Executor {
                            result: XLen::MIN,
                            branch_target_address: cfg.hart.pc_reset_value,
                            jump_target_address: cfg.hart.pc_reset_value,
                            return_target_address: cfg.hart.pc_reset_value,
                            memory_address: XLen::MIN,
                            value_to_store_u8: Default::default(),
                            value_to_store_u16: Default::default(),
                            value_to_store_u32: Default::default(),
                            value_to_store_u64: Default::default(),
                            memory_value_type: MemoryValueType::DOUBLE_WORD,
                            need_to_branch: false,
                            zicsr: ExecutorZicsrContext {
                                csr_value: Default::default(),
                                trap_cause: Default::default(),
                                trap_vector: Default::default(),
                                epc: Default::default(),
                            },
                        },
                    },
                };
                cfg.harts_num as usize
            ],
            bus: SystemBus::new(
                MEM_RESET_VALUE,
                cfg.bus.memory.base_addr,
                cfg.bus.memory.size_bytes,
            ),
        }
    }

    pub fn reset(&mut self, cfg: &SocCfg) {
        for hart in self.harts.iter_mut() {
            hart.reset(cfg);
        }
        self.bus.reset();

        // fill memory with NOP's
        self.bus.fill_with_nops();
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
