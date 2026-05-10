// modules includes
mod config;
mod constants;
pub mod elf_loader;
mod logger;
mod parse_args;
mod soc;
mod utils;

use config::configure;
use logger::init_logger;
use parse_args::parse_args;

// addr: {address:#018X?}

// std lib imports
use std::convert::From;

// external libs imports
use log::{debug, error, info, trace, warn};

// current project imports
use constants::arch::*;
use constants::soc::*;
use soc::cpu::core::decoder::constants::*;
use soc::cpu::core::executor::constants::ExecutableCommand;
use soc::cpu::core::gp_registers::RegisterFile;
use soc::cpu::core::gp_registers::constants::GPRegisterName;
use soc::cpu::core::registers::RegisterFileInterface;
use soc::cpu::extensions::zicsr::constants::CSRRegisterName;
use soc::cpu::extensions::zicsr::{CSRFile, ZicsrContext};
use soc::cpu::priveleged::machine::*;
use soc::memory::{MemoryValue, MemoryValueType};
use soc::memory_bus::{Bus, SystemBus};

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
struct FENCE_Context {
    successor_mem_write: bool,
    successor_mem_read: bool,
    successor_device_output: bool,
    successor_device_input: bool,

    predecessor_mem_write: bool,
    predecessor_mem_read: bool,
    predecessor_device_output: bool,
    predecessor_device_input: bool,

    mode: FenceMode,
}

#[derive(Default, Clone)]
struct Decoder {
    opcode: Opcode,
    source_register_1: XLen,
    source_register_2: XLen,
    destination_register_offset: usize,

    funct3: InstructionType,
    i_funct7: OP_IMM_Funct7,
    r_funct7: Funct7,

    shamt: InstructionType,
    shift_type: OP_IMM_ShiftType,

    immediate: UnsignedImmediate,
    command_to_execute: ExecutableCommand,
    need_to_read_memory: bool,
    need_to_write_memory: bool,
    need_to_write_back: bool,
    need_to_jump: bool,
    need_to_branch: bool,

    fence: FENCE_Context,

    zicsr: ZicsrContext,
}

#[derive(Default, Clone)]
struct ExecutionContext {
    result: XLen,
    branch_target_address: XLen,
    jump_target_address: XLen,
    return_target_address: XLen,
    memory_address: XLen,
    value_to_store_u8: Byte,
    value_to_store_u16: HalfWord,
    value_to_store_u32: Word,
    value_to_store_u64: DoubleWord,
    // value_to_store_u128: u128,
    memory_value_type: MemoryValueType,
}

#[allow(non_snake_case)]
#[derive(Default, Clone)]
pub struct Context {
    gp_register_file: RegisterFile<RISCV_NUM_OF_REGISTERS>,
    PC: XLen,
    control_status_registers: CSRFile<RISCV_NUM_OF_CSR>,
    current_instruction: InstructionType,
    decoder: Decoder,
    execution: ExecutionContext,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub struct RISC_V_CPU {
    context: Context,
}

#[derive(Default)]
pub struct SoC {
    cpu: RISC_V_CPU,
    // memory and memory mapped IO
    bus: SystemBus<BusAddrType>,
}

impl SoC {
    pub fn new() -> Self {
        Self {
            cpu: RISC_V_CPU {
                context: Context {
                    PC: PC_RESET_VALUE,
                    gp_register_file: RegisterFile::new(),
                    control_status_registers: CSRFile::new(),
                    current_instruction: INITIAL_INSTRUCTION,
                    decoder: Decoder {
                        ..Default::default()
                    },
                    execution: ExecutionContext {
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
                    },
                },
            },
            bus: SystemBus::new(MEM_RESET_VALUE, RAM_SIZE),
            // ..Default::default()
        }
    }

    fn fetch(&mut self) {
        trace!(
            "Fetching instruction at addr: {:#018X?}",
            self.cpu.context.PC
        );

        self.cpu.context.current_instruction =
            self.bus.read(self.cpu.context.PC);

        trace!(
            "Fetched instruction: {:#018X?}",
            self.cpu.context.current_instruction
        );
    }

    pub fn reset(&mut self) {
        self.cpu.context.PC = PC_RESET_VALUE;
        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        for i in 1..RISCV_NUM_OF_REGISTERS {
            self.cpu
                .context
                .gp_register_file
                .write(i, RISCV_X0_ZERO_REGISTER_VALUE);
        }
        self.cpu.context.current_instruction = INITIAL_INSTRUCTION;

        // fill memory with NOP's
        self.bus.memory_reset();
        for addr in (RAM_BASE..RAM_END).step_by(RISCV_PC_STEP as usize) {
            self.bus.write(addr, Opcode::OP_IMM as Byte);
        }
    }

    pub fn step(&mut self) {
        trace!("Current PC: {:#018X}", self.cpu.context.PC);

        self.fetch();
        self.cpu.decode();
        self.cpu.execute();
        self.cpu.memory_operation(&mut self.bus);
        self.cpu.write_back();

        // step PC
        self.cpu.advance_pc();
    }

    pub fn ready_for_testing(&mut self) {
        self.bus.ready_for_testing();
    }
}

impl RISC_V_CPU {
    fn set_pc(&mut self, value: XLen) {
        self.context.PC = value;
    }

    fn decode_r_format(&mut self) {
        trace!("Decoding R-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;
        self.context.decoder.funct3 = self.extract_funct3();
        self.context.decoder.r_funct7 = self.extract_r_funct7();

        match self.context.decoder.opcode {
            Opcode::OP => match OP_Funct3::from(self.context.decoder.funct3) {
                OP_Funct3::ARITH => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ADD;
                    }
                    Funct7::F1 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SUB;
                    }
                },
                OP_Funct3::SLL => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLL;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SLT => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLT;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SLTU => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLTU;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::XOR => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::XOR;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SRX => {
                    self.context.decoder.shamt = self.extract_shamt();
                    match self.context.decoder.r_funct7 {
                        Funct7::F0 => {
                            self.context.decoder.command_to_execute =
                                ExecutableCommand::SRL;
                        }
                        Funct7::F1 => {
                            self.context.decoder.command_to_execute =
                                ExecutableCommand::SRA;
                        }
                    }
                }
                OP_Funct3::OR => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::OR;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::AND => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::AND;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
            },
            Opcode::OP_32 => {
                match OP_32_Funct3::from(self.context.decoder.funct3) {
                    OP_32_Funct3::ARITHW => {
                        match self.context.decoder.r_funct7 {
                            Funct7::F0 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::ADDW;
                            }
                            Funct7::F1 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SUBW;
                            }
                        }
                    }
                    OP_32_Funct3::SLLW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLLW;
                    }
                    OP_32_Funct3::SRXW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        match self.context.decoder.r_funct7 {
                            Funct7::F0 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRLW;
                            }
                            Funct7::F1 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRAW;
                            }
                        }
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_i_format(&mut self) {
        trace!("Decoding I-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();

        self.context.decoder.funct3 = self.extract_funct3();

        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        match self.context.decoder.opcode {
            // I_3
            Opcode::LOAD => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                self.context.decoder.need_to_read_memory = true;
                match OPCODE_LOAD_Funct3::from(self.context.decoder.funct3) {
                    OPCODE_LOAD_Funct3::LB => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LB
                    }
                    OPCODE_LOAD_Funct3::LH => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LH
                    }
                    OPCODE_LOAD_Funct3::LW => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LW
                    }
                    OPCODE_LOAD_Funct3::LD => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LD
                    }
                    OPCODE_LOAD_Funct3::LBU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LBU
                    }
                    OPCODE_LOAD_Funct3::LHU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LHU
                    }
                    OPCODE_LOAD_Funct3::LWU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LWU
                    }
                }
            }
            // I_15
            Opcode::MISC_MEM => {
                match MISC_MEM_Funct3::from(self.context.decoder.funct3) {
                    MISC_MEM_Funct3::FENCE => {
                        self.context.decoder.need_to_write_back = false;

                        self.context.decoder.fence.successor_mem_write =
                            self.extract_fence_sw();
                        self.context.decoder.fence.successor_mem_read =
                            self.extract_fence_sr();
                        self.context.decoder.fence.successor_device_output =
                            self.extract_fence_so();
                        self.context.decoder.fence.successor_device_input =
                            self.extract_fence_si();

                        self.context.decoder.fence.predecessor_mem_write =
                            self.extract_fence_pw();
                        self.context.decoder.fence.predecessor_mem_read =
                            self.extract_fence_pr();
                        self.context.decoder.fence.predecessor_device_output =
                            self.extract_fence_po();
                        self.context.decoder.fence.predecessor_device_input =
                            self.extract_fence_pi();

                        self.context.decoder.fence.mode =
                            self.extract_fence_fm();

                        match self.context.decoder.fence.mode {
                            FenceMode::NORMAL => {
                                //
                            }
                            FenceMode::TSO => {
                                //
                            }
                        }

                        self.context.decoder.command_to_execute =
                            ExecutableCommand::FENCE;
                    }

                    MISC_MEM_Funct3::FENCE_I => {
                        self.context.decoder.need_to_write_back = false;

                        //

                        self.context.decoder.command_to_execute =
                            ExecutableCommand::FENCE_I;
                        todo!();
                    }
                }
            }
            // I_19
            Opcode::OP_IMM => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                match OP_IMM_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_Funct3::ADDI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ADDI;
                    }
                    OP_IMM_Funct3::SLLI => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.shift_type =
                            self.extract_shift_type();
                        match self.context.decoder.shift_type {
                            OP_IMM_ShiftType::SXLI => {
                                // funct3 equals SLLI
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SLLI
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_Funct3::SLTI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLTI;
                    }
                    OP_IMM_Funct3::SLTIU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLTIU;
                    }
                    OP_IMM_Funct3::XORI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::XORI;
                    }
                    OP_IMM_Funct3::SRXI => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.shift_type =
                            self.extract_shift_type();
                        match self.context.decoder.shift_type {
                            OP_IMM_ShiftType::SXLI => {
                                // funct3 equals SRLI
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRLI;
                            }
                            OP_IMM_ShiftType::SRAI => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRAI;
                            }
                        }
                    }
                    OP_IMM_Funct3::ORI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ORI;
                    }
                    OP_IMM_Funct3::ANDI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ANDI;
                    }
                }
            }
            // I_27
            Opcode::OP_IMM_32 => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                match OP_IMM_32_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_32_Funct3::ADDIW => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ADDIW
                    }
                    OP_IMM_32_Funct3::SLLIW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SLLIW
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SLLIW
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_32_Funct3::SRXIW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SRXIW
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRLIW
                            }
                            OP_IMM_Funct7::SRAI => {
                                // funct3 equals SRXIW
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRAIW
                            }
                        }
                    }
                }
            }
            // I_103
            Opcode::JALR => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                match JALR_Funct3::from(self.context.decoder.funct3) {
                    JALR_Funct3::JALR => {
                        self.context.decoder.need_to_jump = true;
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::JALR
                    }
                }
            }

            // I_115 (Zicsr_extension)
            Opcode::SYSTEM => {
                self.context.decoder.immediate = self.extract_csr_address();
                match SYSTEM_Funct3::from(self.context.decoder.funct3) {
                    SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM => {
                        self.context.decoder.need_to_write_back = false;
                        let are_rd_and_rs1_zero = || -> bool {
                            (self.context.decoder.source_register_1 == 0)
                                && (self.context.gp_register_file.read(
                                    self.context
                                        .decoder
                                        .destination_register_offset,
                                ) == 0)
                        };
                        let assert_rd_and_rs1_are_zero = || {
                            if !are_rd_and_rs1_zero() {
                                error!("rs1 and rd should be equal 0");
                                unreachable!("rs1 and rd should be equal 0");
                            }
                        };

                        assert_rd_and_rs1_are_zero();
                        match From::from(self.context.decoder.immediate) {
                            SYSTEM_Imm_transfer_control_return_from::ECALL => {
                                self.context.decoder.zicsr.need_to_call_trap =
                                    true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::ECALL;
                            }
                            SYSTEM_Imm_transfer_control_return_from::EBREAK => {
                                //
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::EBREAK;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::URET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::URET;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::SRET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRET;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::MRET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::MRET;
                            }
                        }
                    }
                    // CSR instructions are atomic
                    SYSTEM_Funct3::CSRRW => {
                        self.context.decoder.zicsr.need_to_write_csr = true;
                        if self.context.decoder.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.context.decoder.zicsr.need_to_read_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRW;
                    }
                    SYSTEM_Funct3::CSRRS => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRS;
                    }
                    SYSTEM_Funct3::CSRRC => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRC;
                    }
                    SYSTEM_Funct3::CSRRWI => {
                        self.context.decoder.zicsr.need_to_write_csr = true;
                        self.context.decoder.zicsr.uimm =
                            self.extract_source_register_1_index() as XLen;
                        if self.context.decoder.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.context.decoder.zicsr.need_to_read_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRWI;
                    }
                    SYSTEM_Funct3::CSRRSI => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                            self.context.decoder.zicsr.uimm =
                                self.extract_source_register_1_index() as XLen;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRSI;
                    }
                    SYSTEM_Funct3::CSRRCI => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                            self.context.decoder.zicsr.uimm =
                                self.extract_source_register_1_index() as XLen;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRCI;
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_s_format(&mut self) {
        trace!("Decoding S-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = true;
        self.context.decoder.need_to_write_back = false;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_0();
        self.context.decoder.immediate |=
            self.extract_imm_11_5() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            // S_35
            Opcode::STORE => {
                match STORE_Funct3::from(self.context.decoder.funct3) {
                    STORE_Funct3::SB => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SB;
                    }
                    STORE_Funct3::SH => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SH;
                    }
                    STORE_Funct3::SW => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SW;
                    }
                    STORE_Funct3::SD => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SD;
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_b_format(&mut self) {
        trace!("Decoding B-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = false;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false; // set to true only branch is needed (during exectuion stage)
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_1();
        self.context.decoder.immediate |= self.extract_imm_10_5();
        self.context.decoder.immediate |= self.extract_b_11();
        self.context.decoder.immediate |=
            self.extract_b_12() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            Opcode::BRANCH => {
                match BRANCH_Funct3::from(self.context.decoder.funct3) {
                    BRANCH_Funct3::BEQ => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BEQ
                    }
                    BRANCH_Funct3::BNE => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BNE
                    }
                    BRANCH_Funct3::BLT => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BLT
                    }
                    BRANCH_Funct3::BGE => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BGE
                    }
                    BRANCH_Funct3::BLTU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BLTU
                    }
                    BRANCH_Funct3::BGEU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BGEU
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_u_format(&mut self) {
        trace!("Decoding U-type instruction");

        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate =
            self.extract_imm_31_12() as UnsignedImmediate;

        match self.context.decoder.opcode {
            Opcode::AUIPC => {
                self.context.decoder.command_to_execute =
                    ExecutableCommand::AUIPC;
            }
            Opcode::LUI => {
                self.context.decoder.command_to_execute =
                    ExecutableCommand::LUI;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_j_format(&mut self) {
        trace!("Decoding J-type instruction");

        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = true;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_10_1();
        self.context.decoder.immediate |= self.extract_j_11();
        self.context.decoder.immediate |= self.extract_imm_19_12();
        self.context.decoder.immediate |=
            self.extract_j_20() as UnsignedImmediate;

        match self.context.decoder.opcode {
            Opcode::JAL => {
                self.context.decoder.command_to_execute =
                    ExecutableCommand::JAL;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_r4_format(&mut self) {
        trace!("Decoding R4-type instruction");

        //
        match self.context.decoder.opcode {
            //
            _ => match self.context.decoder.funct3 {
                //
                _ => {
                    todo!("TODO: unimplemented");
                }
            },
        };
    }

    fn decode(&mut self) {
        self.context.decoder.opcode = self.extract_opcode();

        match From::from(self.context.decoder.opcode.clone()) {
            OpcodeToInstructionFormat::R_51_OP
            | OpcodeToInstructionFormat::R_59_OP_32
            | OpcodeToInstructionFormat::R_83_OP_FP => self.decode_r_format(),
            OpcodeToInstructionFormat::I_3_LOAD
            | OpcodeToInstructionFormat::I_7_LOAD_FP
            | OpcodeToInstructionFormat::I_19_OP_IMM
            | OpcodeToInstructionFormat::I_27_OP_IMM_32
            | OpcodeToInstructionFormat::I_103_JALR
            | OpcodeToInstructionFormat::I_115_SYSTEM // (Zicsr_extension)
            | OpcodeToInstructionFormat::I_15_MISC_MEM // FENCE, Zifencei
            => self.decode_i_format(),
            OpcodeToInstructionFormat::S_35_STORE
            | OpcodeToInstructionFormat::S_39_STORE_FP => {
                self.decode_s_format()
            }
            OpcodeToInstructionFormat::B_99_BRANCH => self.decode_b_format(),
            OpcodeToInstructionFormat::U_23_AUIPC
            | OpcodeToInstructionFormat::U_55_LUI => self.decode_u_format(),
            OpcodeToInstructionFormat::J_111_JAL => self.decode_j_format(),
            OpcodeToInstructionFormat::R4_67_MADD
            | OpcodeToInstructionFormat::R4_71_MSUB
            | OpcodeToInstructionFormat::R4_75_NMSUB
            | OpcodeToInstructionFormat::R4_79_NMADD => self.decode_r4_format(),
        }
    }

    fn execute(&mut self) {
        match self.context.decoder.command_to_execute {
            // RV32I
            // I_3
            ExecutableCommand::LB => self.instruction_load_byte(),
            ExecutableCommand::LH => self.instruction_load_half(),
            ExecutableCommand::LW => self.instruction_load_word(),
            ExecutableCommand::LBU => self.instruction_load_byte_unsigned(),
            ExecutableCommand::LHU => self.instruction_load_half_unsigned(),
            // I_19
            ExecutableCommand::ADDI => self.instruction_addi(),
            ExecutableCommand::SLLI => self.instruction_slli(),
            ExecutableCommand::SLTI => self.instruction_slti(),
            ExecutableCommand::SLTIU => self.instruction_sltiu(),
            ExecutableCommand::XORI => self.instruction_xori(),
            ExecutableCommand::SRLI => self.instruction_srli(),
            ExecutableCommand::SRAI => self.instruction_srai(),
            ExecutableCommand::ORI => self.instruction_ori(),
            ExecutableCommand::ANDI => self.instruction_andi(),
            // U_23
            ExecutableCommand::AUIPC => self.instruction_auipc(),
            // S_35
            ExecutableCommand::SB => self.instruction_store_byte(),
            ExecutableCommand::SH => self.instruction_store_half_word(),
            ExecutableCommand::SW => self.instruction_store_word(),
            // R_51
            ExecutableCommand::ADD => self.instruction_add(),
            ExecutableCommand::SUB => self.instruction_sub(),
            ExecutableCommand::SLL => self.instruction_sll(),
            ExecutableCommand::SLT => self.instruction_slt(),
            ExecutableCommand::SLTU => self.instruction_sltu(),
            ExecutableCommand::XOR => self.instruction_xor(),
            ExecutableCommand::SRL => self.instruction_srl(),
            ExecutableCommand::SRA => self.instruction_sra(),
            ExecutableCommand::OR => self.instruction_or(),
            ExecutableCommand::AND => self.instruction_and(),
            // U_55
            ExecutableCommand::LUI => self.instruction_lui(),
            // B_99
            ExecutableCommand::BEQ => self.instruction_beq(),
            ExecutableCommand::BNE => self.instruction_bne(),
            ExecutableCommand::BLT => self.instruction_blt(),
            ExecutableCommand::BGE => self.instruction_bge(),
            ExecutableCommand::BLTU => self.instruction_bltu(),
            ExecutableCommand::BGEU => self.instruction_bgeu(),
            // I_103
            ExecutableCommand::JALR => self.instruction_jalr(),
            // J_111
            ExecutableCommand::JAL => self.instruction_jal(),

            // RV64I
            // I_3
            ExecutableCommand::LD => self.instruction_load_double_word(),
            ExecutableCommand::LWU => self.instruction_load_word_unsigned(),
            // I_27
            ExecutableCommand::ADDIW => {
                self.instruction_add_immediate_word();
            }
            ExecutableCommand::SLLIW => {
                self.instruction_shift_left_logical_immediate_word();
            }
            ExecutableCommand::SRLIW => {
                self.instruction_shift_right_logical_immediate_word();
            }
            ExecutableCommand::SRAIW => {
                self.instruction_shift_right_arith_immediate_word();
            }
            // S_35
            ExecutableCommand::SD => self.instruction_store_double_word(),
            // R_59
            ExecutableCommand::ADDW => self.instruction_add_word(),
            ExecutableCommand::SUBW => self.instruction_sub_word(),
            ExecutableCommand::SLLW => {
                self.instruction_shift_left_logical_word();
            }
            ExecutableCommand::SRLW => {
                self.instruction_shift_right_logical_word();
            }
            ExecutableCommand::SRAW => {
                self.instruction_shift_right_arith_word();
            }

            ExecutableCommand::FENCE => {
                self.instruction_fence();
            }

            ExecutableCommand::FENCE_I => {
                self.instruction_fence_i();
            }

            ExecutableCommand::ECALL => {
                self.instruction_execution_call();
            }
            ExecutableCommand::EBREAK => {
                self.instruction_execution_break();
            }

            ExecutableCommand::URET => {
                self.instruction_user_return();
            }
            ExecutableCommand::SRET => {
                self.instruction_supervisor_return();
            }
            ExecutableCommand::MRET => {
                self.instruction_machine_return();
            }

            // Zicsr extension
            ExecutableCommand::CSRRW => {
                self.instruction_csr_read_write();
            }
            ExecutableCommand::CSRRS => {
                self.instruction_csr_read_set();
            }
            ExecutableCommand::CSRRC => {
                self.instruction_csr_read_clear();
            }
            ExecutableCommand::CSRRWI => {
                self.instruction_csr_read_write_immediate();
            }
            ExecutableCommand::CSRRSI => {
                self.instruction_csr_read_set_immediate();
            }
            ExecutableCommand::CSRRCI => {
                self.instruction_csr_read_clear_immediate();
            }
        }
    }

    fn instruction_load_byte(&mut self) {
        trace!("Executing LB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_load_half(&mut self) {
        trace!("Executing LH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_load_word(&mut self) {
        trace!("Executing LW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_load_byte_unsigned(&mut self) {
        trace!("Executing LBU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_BYTE;
    }

    fn instruction_load_half_unsigned(&mut self) {
        trace!("Executing LHU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_HALF_WORD;
    }

    fn instruction_addi(&mut self) {
        trace!("Executing ADDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_slli(&mut self) {
        trace!("Executing SLLI...");

        self.context.execution.result = self.context.decoder.source_register_1
            << self.context.decoder.shamt;
    }

    fn instruction_slti(&mut self) {
        trace!("Executing SLTI...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as SignedXLen)
                < (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_sltiu(&mut self) {
        trace!("Executing SLTIU...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as UnsignedXLen)
                < (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as UnsignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_xori(&mut self) {
        trace!("Executing XORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_srli(&mut self) {
        trace!("Executing SRLI...");

        self.context.execution.result = self.context.decoder.source_register_1
            >> self.context.decoder.shamt;
    }

    fn instruction_srai(&mut self) {
        trace!("Executing SRAI...");

        self.context.execution.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.shamt) as XLen;
    }

    fn instruction_ori(&mut self) {
        trace!("Executing ORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            | self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_andi(&mut self) {
        trace!("Executing ANDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_auipc(&mut self) {
        trace!("Executing AUIPC...");

        self.context.execution.result = self.context.PC
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_store_byte(&mut self) {
        trace!("Executing SB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u8 =
            self.context.decoder.source_register_2 as Byte;
        self.context.execution.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_store_half_word(&mut self) {
        trace!("Executing SH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u16 =
            self.context.decoder.source_register_2 as HalfWord;
        self.context.execution.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_store_word(&mut self) {
        trace!("Executing SW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u32 =
            self.context.decoder.source_register_2 as Word;
        self.context.execution.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_add(&mut self) {
        trace!("Executing ADD...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.source_register_2;
    }

    fn instruction_sub(&mut self) {
        trace!("Executing SUB...");

        self.context.execution.result = self.context.decoder.source_register_1
            - self.context.decoder.source_register_2;
    }

    fn instruction_sll(&mut self) {
        trace!("Executing SLL...");

        self.context.execution.result = self.context.decoder.source_register_1
            << self.context.decoder.source_register_2;
    }

    fn instruction_slt(&mut self) {
        trace!("Executing SLT...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as SignedXLen)
                < (self.context.decoder.source_register_2 as SignedXLen)
            {
                1
            } else {
                0
            }
    }

    fn instruction_sltu(&mut self) {
        trace!("Executing SLTU...");

        self.context.execution.result =
            if self.context.decoder.source_register_1
                < self.context.decoder.source_register_2
            {
                1
            } else {
                0
            }
    }

    fn instruction_xor(&mut self) {
        trace!("Executing XOR...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.source_register_2;
    }

    fn instruction_srl(&mut self) {
        trace!("Executing SRL...");

        self.context.execution.result = self.context.decoder.source_register_1
            >> self.context.decoder.source_register_2;
    }

    fn instruction_sra(&mut self) {
        trace!("Executing SRA...");

        self.context.execution.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.source_register_2) as XLen;
    }

    fn instruction_or(&mut self) {
        trace!("Executing OR...");

        self.context.execution.result = self.context.decoder.source_register_1
            | self.context.decoder.source_register_2;
    }

    fn instruction_and(&mut self) {
        trace!("Executing AND...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.source_register_2;
    }

    fn instruction_lui(&mut self) {
        trace!("Executing LUI...");

        self.context.execution.result = self.context.decoder.immediate
            as SignedImmediate
            as SignedXLen as XLen;
    }

    fn instruction_beq(&mut self) {
        trace!("Executing BEQ...");

        if self.context.decoder.source_register_1
            == self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bne(&mut self) {
        trace!(
            "Executing BNE: {:#018X}, {:#018X}...",
            self.context.decoder.source_register_1,
            self.context.decoder.source_register_2
        );

        if self.context.decoder.source_register_1
            != self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        } else {
            trace!("Skip BNE...");
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_blt(&mut self) {
        trace!("Executing BLT...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            < (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bge(&mut self) {
        trace!("Executing BGE...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            >= (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bltu(&mut self) {
        trace!("Executing BLTU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            < (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bgeu(&mut self) {
        trace!("Executing BGEU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            >= (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jalr(&mut self) {
        trace!("Executing JALR...");

        self.context.execution.jump_target_address =
            (self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen))
                & !(XLen::MAX & 0x01);
        self.context.execution.result = self.context.PC + RISCV_PC_STEP;
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jal(&mut self) {
        trace!("Executing JAL...");

        self.context.execution.jump_target_address = self.context.PC
            + (self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen);
        self.context.execution.result = self.context.PC + RISCV_PC_STEP;
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_load_double_word(&mut self) {
        trace!("Executing LD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_load_word_unsigned(&mut self) {
        trace!("Executing LWU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_WORD;
    }

    fn instruction_add_immediate_word(&mut self) {
        trace!("Executing ADDIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            + self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_immediate_word(&mut self) {
        trace!("Executing SLLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_immediate_word(&mut self) {
        trace!("Executing SRLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_arith_immediate_word(&mut self) {
        trace!("Executing SRAIW...");

        let result32 = (self.context.decoder.source_register_1 as SignedXLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_store_double_word(&mut self) {
        trace!("Executing SD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u64 =
            self.context.decoder.source_register_2;
        self.context.execution.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_add_word(&mut self) {
        trace!("Executing ADDW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            + self.context.decoder.source_register_2 as SignedXLen32;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_sub_word(&mut self) {
        trace!("Executing SUBW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            - self.context.decoder.source_register_2 as SignedXLen32;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_word(&mut self) {
        trace!("Executing SLLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.shamt;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_word(&mut self) {
        trace!("Executing SRLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.shamt;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_arith_word(&mut self) {
        trace!("Executing SRAW...");

        let result32 = (self.context.decoder.source_register_1 as SignedXLen32)
            >> self.context.decoder.shamt;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_fence(&mut self) {
        trace!("Executing FENCE...");

        //
        error!("TODO: unimplemented instruction");
    }

    fn instruction_fence_i(&mut self) {
        trace!("Executing FENCE_I...");

        //
        error!("TODO: unimplemented instruction");
        todo!();
    }

    fn instruction_execution_call(&mut self) {
        trace!("Executing ECALL...");

        let current_privilege_mode = self.read_mstatus_mpp();
        match current_privilege_mode {
            MStatus_MPP::USER => {
                self.context.decoder.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_U_MODE;
                self.context.decoder.zicsr.trap_vector = self
                    .context
                    .control_status_registers
                    .read(CSRRegisterName::mtvec as usize);
            }
            MStatus_MPP::SUPERVISOR => {
                self.context.decoder.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_S_MODE;
                todo!();
            }
            MStatus_MPP::MACHINE => {
                self.context.decoder.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_M_MODE;
                self.context.decoder.zicsr.trap_vector = self
                    .context
                    .control_status_registers
                    .read(CSRRegisterName::mtvec as usize);
            }

            _ => {
                error!("unimplemented!");
                todo!("unimplemented!");
            }
        }
        self.context.decoder.zicsr.epc = self.context.PC;
    }

    fn instruction_execution_break(&mut self) {
        trace!("Executing EBREAK...");

        //
        todo!();
    }

    fn instruction_user_return(&mut self) {
        trace!("Executing URET...");

        //
        todo!();
    }

    fn instruction_supervisor_return(&mut self) {
        trace!("Executing SRET...");

        //
        todo!();
    }

    fn instruction_machine_return(&mut self) {
        trace!("Executing MRET...");

        self.context.execution.return_target_address = self
            .context
            .control_status_registers
            .read(CSRRegisterName::mepc as usize);
    }

    fn read_csr_to_result(&mut self) {
        self.context.execution.result = self
            .context
            .control_status_registers
            .read(self.context.decoder.immediate as usize);
    }

    fn prepare_value_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value = value;
    }

    fn prepare_set_bits_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value |= value;
    }

    fn prepare_clear_bits_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value &= !value;
    }

    fn instruction_csr_read_write(&mut self) {
        trace!("Executing CSRRW...");

        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.source_register_1);
    }

    fn instruction_csr_read_set(&mut self) {
        trace!("Executing CSRRS...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_clear(&mut self) {
        trace!("Executing CSRRC...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_write_immediate(&mut self) {
        trace!("Executing CSRRWI...");

        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.zicsr.uimm);
    }

    fn instruction_csr_read_set_immediate(&mut self) {
        trace!("Executing CSRRSI...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

    fn instruction_csr_read_clear_immediate(&mut self) {
        trace!("Executing CSRRCI...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

    fn read_memory_signed<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context.execution.result = bus
            .read::<ValType>(self.context.execution.memory_address)
            .sign_extend();
    }

    fn read_memory_unsigned<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context.execution.result = bus
            .read::<ValType>(self.context.execution.memory_address)
            .zero_extend();
    }

    fn memory_operation(&mut self, bus: &mut SystemBus<BusAddrType>) {
        if self.context.decoder.need_to_read_memory {
            trace!("Executing memory read...");

            match self.context.execution.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    self.read_memory_signed::<DoubleWord>(bus);
                }
                MemoryValueType::WORD => {
                    self.read_memory_signed::<Word>(bus);
                }
                MemoryValueType::HALF_WORD => {
                    self.read_memory_signed::<HalfWord>(bus);
                }
                MemoryValueType::BYTE => {
                    self.read_memory_signed::<Byte>(bus);
                }
                MemoryValueType::UNSIGNED_WORD => {
                    self.read_memory_unsigned::<Word>(bus);
                }
                MemoryValueType::UNSIGNED_HALF_WORD => {
                    self.read_memory_unsigned::<HalfWord>(bus);
                }
                MemoryValueType::UNSIGNED_BYTE => {
                    self.read_memory_unsigned::<Byte>(bus);
                }
            }
        } else if self.context.decoder.need_to_write_memory {
            trace!("Executing memory write...");

            match self.context.execution.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u64,
                    );
                }
                MemoryValueType::WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u32,
                    );
                }
                MemoryValueType::HALF_WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u16,
                    );
                }
                MemoryValueType::BYTE => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u8,
                    );
                }

                MemoryValueType::UNSIGNED_WORD
                | MemoryValueType::UNSIGNED_HALF_WORD
                | MemoryValueType::UNSIGNED_BYTE => {
                    error!("unreachable!");
                    unreachable!("unreachable!");
                }
            }
        }
    }

    fn write_back_to_reg(&mut self) {
        let register_offset = self.context.decoder.destination_register_offset;
        trace!(
            "Writing back to register x{register_offset:#?},\
            value: {:#018X?}...",
            self.context.execution.result
        );

        self.context
            .gp_register_file
            .write(register_offset, self.context.execution.result);
    }

    fn set_pc_to_target_address(&mut self, target_addr: XLen) {
        error!(
            "TODO: unimplemented: generate exception if BRANCH/JUMP (BTA/JTA) address is misaligned"
        );
        self.context.PC = target_addr;
    }

    fn return_to_target_address(&mut self) {
        trace!(
            "Returning from trap to target address: {:#018X?}...",
            self.context.execution.return_target_address
        );

        self.set_pc_to_target_address(
            self.context.execution.return_target_address,
        );
    }

    fn branch_to_target_address(&mut self) {
        trace!(
            "Updating PC to BTA: {:#018X?}...",
            self.context.execution.branch_target_address
        );

        self.set_pc_to_target_address(
            self.context.execution.branch_target_address,
        );
    }

    fn jump_to_target_address(&mut self) {
        trace!(
            "Updating PC to JTA: {:#018X?}...",
            self.context.execution.jump_target_address
        );

        self.set_pc_to_target_address(
            self.context.execution.jump_target_address,
        );
    }

    fn trap_to_target_address(&mut self) {
        trace!(
            "Calling trap handler at addr: {:#018X?}...",
            self.context.decoder.zicsr.trap_vector
        );

        self.set_pc_to_target_address(self.context.decoder.zicsr.trap_vector);
    }

    fn call_trap_handler(&mut self) {
        match self.context.decoder.zicsr.trap_cause {
            MCause_Code::EXCEPTION_ENV_CALL_FROM_U_MODE => {
                self.context.control_status_registers.write(
                    CSRRegisterName::mcause as usize,
                    self.context.decoder.zicsr.trap_cause.clone() as XLen,
                );
                self.context.control_status_registers.write(
                    CSRRegisterName::mepc as usize,
                    self.context.decoder.zicsr.epc as XLen,
                );
                error!("Probably implemented incorrectly");
            }
            MCause_Code::EXCEPTION_ENV_CALL_FROM_S_MODE => {
                todo!();
            }
            MCause_Code::EXCEPTION_ENV_CALL_FROM_M_MODE => {
                self.context.control_status_registers.write(
                    CSRRegisterName::mcause as usize,
                    self.context.decoder.zicsr.trap_cause.clone() as XLen,
                );
                self.context.control_status_registers.write(
                    CSRRegisterName::mepc as usize,
                    self.context.decoder.zicsr.epc as XLen,
                );
            }

            _ => {
                error!("unimplemented!");
                todo!("unimplemented!");
            }
        }

        self.trap_to_target_address();
    }

    fn write_to_csr(&mut self) {
        let register_offset = self.context.decoder.immediate as usize;
        trace!("Writing to CSR #{register_offset:#?}...");

        self.context
            .control_status_registers
            .write(register_offset, self.context.decoder.zicsr.csr_value);
    }

    fn write_back(&mut self) {
        // check if write to general purpose register is necessary
        if self.context.decoder.need_to_write_back {
            self.write_back_to_reg();
            // jumps always incur register write-back
            if self.context.decoder.need_to_jump {
                self.jump_to_target_address();
                return;
            }
        } else if self.context.decoder.need_to_branch {
            self.branch_to_target_address();
            return;
        }
        // check if write to CSR is necessary
        if self.context.decoder.zicsr.need_to_write_csr {
            self.write_to_csr();
        }
        // check if return from trap is needed
        else if self.context.decoder.zicsr.need_to_return_from_trap {
            self.return_to_target_address();
            return;
        }
        if self.context.decoder.zicsr.need_to_call_trap {
            self.call_trap_handler();
        }
    }

    fn is_need_to_advance_pc(&self) -> bool {
        if !self.context.decoder.need_to_jump
            && !self.context.decoder.need_to_branch
            && !self.context.decoder.zicsr.need_to_return_from_trap
            && !self.context.decoder.zicsr.need_to_call_trap
        {
            true
        } else {
            false
        }
    }

    fn advance_pc(&mut self) {
        if self.is_need_to_advance_pc() {
            trace!("Advancing PC...");

            self.context.PC += RISCV_PC_STEP;
        } else {
            trace!("Skip advancing PC...");
        }
    }

    fn extract_opcode(&self) -> Opcode {
        Opcode::from(
            ((self.context.current_instruction >> BIT_OFFSET_OPCODE)
                & MASK_OPCODE) as u8,
        )
    }

    fn extract_source_register_1_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_1)
            & MASK_SOURCE_REGISTER_1) as usize
    }

    fn extract_source_register_1(&self) -> XLen {
        self.context
            .gp_register_file
            .read(self.extract_source_register_1_index())
    }

    fn extract_source_register_2_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_2)
            & MASK_SOURCE_REGISTER_2) as usize
    }

    fn extract_source_register_2(&self) -> XLen {
        self.context
            .gp_register_file
            .read(self.extract_source_register_2_index())
    }

    fn extract_destination_register_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_DESTINATION_REGISTER)
            & MASK_DESTINATION_REGISTER) as usize
    }

    fn extract_funct3(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_FUNCT3) & MASK_FUNCT3
    }

    // fn extract_i_funct7(&self) -> OP_IMM_Funct7 {
    //     From::from(
    //         (self.context.current_instruction >> BIT_OFFSET_I_FUNCT7)
    //             & MASK_I_FUNCT7,
    //     )
    // }

    fn extract_shift_type(&self) -> OP_IMM_ShiftType {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_SHTYPE)
                & MASK_SHTYPE,
        )
    }

    fn extract_r_funct7(&self) -> Funct7 {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_R_FUNCT7)
                & MASK_R_FUNCT7,
        )
    }

    // fn extract_shamt_rv32(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT_RV32
    // }

    fn extract_shamt(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT
    }

    fn extract_imm_11_0(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL))
            as SignedXLen
    }

    fn extract_fence_sw(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SW) != 0
    }

    fn extract_fence_sr(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SR) != 0
    }

    fn extract_fence_so(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SO) != 0
    }

    fn extract_fence_si(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SI) != 0
    }

    fn extract_fence_pw(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PW) != 0
    }

    fn extract_fence_pr(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PR) != 0
    }

    fn extract_fence_po(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PO) != 0
    }

    fn extract_fence_pi(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PI) != 0
    }

    fn extract_fence_fm(&self) -> FenceMode {
        FenceMode::from(
            (self.context.current_instruction & MASK_I_FENCE_FM)
                >> BIT_OFFSET_I_FENCE_FM,
        )
    }

    fn extract_csr_address(&self) -> UnsignedImmediate {
        self.context.current_instruction
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL)
    }

    fn extract_imm_11_5(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_5 - BIT_OFFSET_IMM_11_5_FINAL))
            as SignedXLen
            & !((1 << BIT_OFFSET_IMM_11_5_FINAL) - 1)
    }

    fn extract_imm_4_0(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_0 - BIT_OFFSET_IMM_4_0_FINAL))
            & MASK_IMM_4_0
    }

    fn extract_imm_31_12(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_31_12 - BIT_OFFSET_IMM_31_12_FINAL))
            as SignedXLen
            & !((1 << BIT_OFFSET_IMM_31_12_FINAL) - 1)
    }

    fn extract_imm_10_5(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_5 - BIT_OFFSET_IMM_10_5_FINAL))
            & MASK_IMM_10_5
    }

    fn extract_imm_4_1(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_1 - BIT_OFFSET_IMM_4_1_FINAL))
            & MASK_IMM_4_1
    }

    fn extract_imm_10_1(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_1 - BIT_OFFSET_IMM_10_1_FINAL))
            & MASK_IMM_10_1
    }

    fn extract_imm_19_12(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_19_12 - BIT_OFFSET_IMM_19_12_FINAL))
            & MASK_IMM_19_12
    }

    fn extract_b_11(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            << (BIT_OFFSET_B_11_FINAL - BIT_OFFSET_B_11))
            & MASK_B_11
    }

    fn extract_b_12(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_B_12 - BIT_OFFSET_B_12_FINAL)) as SignedXLen
            & !((1 << BIT_OFFSET_B_12_FINAL) - 1)
    }

    fn extract_j_11(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_J_11 - BIT_OFFSET_J_11_FINAL))
            & MASK_J_11
    }

    fn extract_j_20(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        ((self.context.current_instruction as SignedInstructionType)
            >> (BIT_OFFSET_J_20 - BIT_OFFSET_J_20_FINAL)) as SignedXLen
            & !((1 << BIT_OFFSET_J_20_FINAL) - 1)
    }

    // privileged mode extractors
    fn read_mstatus_mpp(&self) -> MStatus_MPP {
        let mstatus = self
            .context
            .control_status_registers
            .read(CSRRegisterName::mstatus as usize);
        MStatus_MPP::from((mstatus >> MSTATUS_MPP_OFFSET) & MSTATUS_MPP_MASK)
    }
}

pub fn init() {
    init_logger();
    parse_args();
    configure();
}
