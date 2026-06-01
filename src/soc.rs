pub mod cpu;
pub mod memory;
pub mod memory_bus;

use self::cpu::RISC_V_CPU;
use self::cpu::core::Context;
use self::cpu::core::decoder::{Decoder, constants::Opcode};
use self::cpu::core::executor::Executor;
use self::cpu::core::fetch_unit::FetchUnit;
use self::cpu::core::gp_registers::{Register, RegisterFile};
use self::cpu::core::registers::{RegisterFileInterface, RegisterInterface};
use self::cpu::extensions::zicsr::CSRFile;
use self::memory::MemoryValueType;
use self::memory_bus::{Bus, SystemBus};

use crate::constants::arch::Byte;
use crate::constants::arch::{
    BusAddrType, RISCV_NUM_OF_REGISTERS, RISCV_PC_STEP,
    RISCV_X0_ZERO_REGISTER_VALUE, XLen,
};
use crate::constants::soc::{
    INITIAL_INSTRUCTION, MEM_RESET_VALUE, PC_RESET_VALUE, RAM_BASE, RAM_END,
    RAM_SIZE,
};
use crate::soc::cpu::extensions::zicsr::ExecutorZicsrContext;

use log::trace;

pub struct SoC {
    pub(crate) cpu: RISC_V_CPU,
    // memory and memory mapped IO
    pub(crate) bus: SystemBus<BusAddrType>,
}

impl SoC {
    pub fn new() -> Self {
        Self {
            cpu: RISC_V_CPU {
                context: Context {
                    PC: Register::new(PC_RESET_VALUE),
                    gp_register_file: RegisterFile::new(),
                    control_status_registers: CSRFile::new(),
                    fetch_unit: FetchUnit::new(INITIAL_INSTRUCTION),
                    decoder: Decoder {
                        ..Default::default()
                    },
                    executor: Executor {
                        result: XLen::MIN,
                        branch_target_address: PC_RESET_VALUE,
                        jump_target_address: PC_RESET_VALUE,
                        return_target_address: PC_RESET_VALUE,
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
            bus: SystemBus::new(MEM_RESET_VALUE, RAM_SIZE),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();

        self.cpu.context.PC.reset(); // = PC_RESET_VALUE;
        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        for i in 1..RISCV_NUM_OF_REGISTERS {
            self.cpu
                .context
                .gp_register_file
                .write(i, RISCV_X0_ZERO_REGISTER_VALUE);
        }
        self.cpu.context.fetch_unit.current_instruction = INITIAL_INSTRUCTION;

        // fill memory with NOP's
        self.bus.memory_reset();
        for addr in (RAM_BASE..RAM_END).step_by(RISCV_PC_STEP as usize) {
            self.bus.write(addr, Opcode::OP_IMM as Byte);
        }
    }

    pub fn step(&mut self) {
        trace!("Current PC: {:#018X}", self.cpu.context.PC.read());

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
