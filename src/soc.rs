pub mod cpu;
pub mod memory;
pub mod memory_bus;

use crate::config::SocCfg;

use self::cpu::Hart;
use self::cpu::core::decoder::{Decoder, constants::InstructionSize};
use self::cpu::core::executor::Executor;
use self::cpu::core::fetch_unit::FetchUnit;
use self::cpu::core::gp_registers::{Register, RegisterFile};
use self::cpu::core::registers::{RegisterFileInterface, RegisterInterface};
use self::cpu::core::{ArchState, Context};
use self::cpu::extensions::zicsr::CSRFile;
use self::cpu::extensions::zicsr::ExecutorZicsrContext;
use self::memory::MemoryValueType;
use self::memory_bus::{Bus, SystemBus};

use super::constants::arch::{BusAddrType, XLen};
use super::constants::soc::{INITIAL_INSTRUCTION, MEM_RESET_VALUE};

use log::trace;

pub struct SoC {
    pub(crate) cpu: Hart,
    // memory and memory mapped IO
    // von Neumann architechture, so common bus is used for instructions/data
    pub(crate) bus: SystemBus<BusAddrType>,
}

impl SoC {
    pub fn new(cfg: &SocCfg) -> Self {
        Self {
            cpu: Hart {
                context: Context {
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
            },
            bus: SystemBus::new(
                MEM_RESET_VALUE,
                cfg.bus.memory.base_addr,
                cfg.bus.memory.size_bytes,
            ),
        }
    }

    pub fn reset(&mut self, cfg: &SocCfg) {
        self.cpu.reset(cfg);
        self.bus.reset();

        // fill memory with NOP's
        self.bus.fill_with_nops();
    }

    pub fn step(&mut self) {
        trace!(
            "Current PC: {:#018X}",
            self.cpu.context.arch_state.PC.read()
        );

        self.cpu.fetch(&self.bus);
        self.cpu.decode();
        self.cpu.execute();
        self.cpu.memory_operation(&mut self.bus);
        self.cpu.write_back();

        // step PC
        self.cpu.advance_pc();
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
