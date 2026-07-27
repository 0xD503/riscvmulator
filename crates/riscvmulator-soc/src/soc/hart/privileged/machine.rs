// TODO: disallow dead code
#![allow(dead_code)]

// use std::sync::atomic::AtomicU64;

use log::error;

use crate::constants::arch::XLen;

pub type MXLen = u64;
// pub type AtomicMXLen = AtomicU64;
pub type SignedMXLen = i64;
pub type UnsignedMXLen = u64;

pub const M_REGISTER_NOT_IMPLEMENTED: MXLen = 0;

pub const MISA_EXTENSIONS_A_ATOMIC: MXLen = 0x0000_0001;
pub const MISA_EXTENSIONS_B_B: MXLen = 0x0000_0002;
pub const MISA_EXTENSIONS_C_COMPRESSED: MXLen = 0x0000_0004;
pub const MISA_EXTENSIONS_D_FP_DOUBLE: MXLen = 0x0000_0008;
pub const MISA_EXTENSIONS_E_EMBEDDED: MXLen = 0x0000_0010;
pub const MISA_EXTENSIONS_F_FP_FLOAT: MXLen = 0x0000_0020;
pub const MISA_EXTENSIONS_G_RESERVED: MXLen = 0x0000_0040;
pub const MISA_EXTENSIONS_H_HYPERVISOR: MXLen = 0x0000_0080;
pub const MISA_EXTENSIONS_I_BASE_INTEGER: MXLen = 0x0000_0100;
pub const MISA_EXTENSIONS_J_RESERVED: MXLen = 0x0000_0200;
pub const MISA_EXTENSIONS_K_RESERVED: MXLen = 0x0000_0400;
pub const MISA_EXTENSIONS_L_RESERVED: MXLen = 0x0000_0800;
pub const MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT: MXLen = 0x0000_1000;
pub const MISA_EXTENSIONS_N_USER_LEVEL_INTERRUPTS: MXLen = 0x0000_2000;
pub const MISA_EXTENSIONS_O_RESERVED: MXLen = 0x0000_4000;
pub const MISA_EXTENSIONS_P_PACKED_SIMD: MXLen = 0x0000_8000;
pub const MISA_EXTENSIONS_Q_QUAD_PRECISION_FP: MXLen = 0x0001_0000;
pub const MISA_EXTENSIONS_R_RESERVED: MXLen = 0x0002_0000;
pub const MISA_EXTENSIONS_S_SUPERVISOR_MODE: MXLen = 0x0004_0000;
pub const MISA_EXTENSIONS_T_RESERVED: MXLen = 0x0008_0000;
pub const MISA_EXTENSIONS_U_USER_MODE: MXLen = 0x0010_0000;
pub const MISA_EXTENSIONS_V_VECTOR: MXLen = 0x0020_0000;
pub const MISA_EXTENSIONS_W_RESERVED: MXLen = 0x0040_0000;
pub const MISA_EXTENSIONS_X_NON_STANDART: MXLen = 0x0080_0000;
pub const MISA_EXTENSIONS_Y_RESERVED: MXLen = 0x0100_0000;
pub const MISA_EXTENSIONS_Z_RESERVED: MXLen = 0x0200_0000;

pub const MISA_MXL_OFFSET: usize = (MXLen::BITS - 2) as usize;

pub mod misa {
    use super::MXLen;
    use crate::constants::arch::XLen;
    // use crate::constants::arch::RISCV_NUM_OF_CSR;
    use crate::soc::hart::core::registers::{
        RawRegisterFileInterface, RegisterFileInterface,
    };
    use crate::soc::hart::extensions::zicsr::CSRFile;
    use crate::soc::hart::extensions::zicsr::constants::CSRRegisterName;

    pub fn set_misa<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write(CSRRegisterName::misa as usize, value);
    }

    pub fn set_misa_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write_raw(CSRRegisterName::misa as usize, value);
    }

    pub fn set_mstatus<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write(CSRRegisterName::mstatus as usize, value);
    }

    pub fn set_mstatus_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write_raw(CSRRegisterName::mstatus as usize, value);
    }

    pub fn set_mstatush<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write(CSRRegisterName::mstatush as usize, value);
    }

    pub fn set_mstatush_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write_raw(CSRRegisterName::mstatush as usize, value);
    }

    pub fn set_mhartid<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        id: usize,
    ) {
        csrs.write(CSRRegisterName::mhartid as usize, id as XLen);
    }

    pub fn set_mhartid_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        id: usize,
    ) {
        csrs.write_raw(CSRRegisterName::mhartid as usize, id as XLen);
    }
}

pub const MSTATUS_MPP_OFFSET: usize = 11;
pub const MSTATUS_MPP_MASK: XLen = 0b11;

#[allow(non_camel_case_types)]
pub enum MISA_MXL {
    MXL_unimplemented = 0,
    XLEN_32 = 1,
    XLEN_64 = 2,
    reserved = 3,
}

// fn extract_mxl() -> MXLen {
//     //
// }

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub enum MStatus_MPP {
    USER = 0,
    SUPERVISOR = 1,
    HYPERVISOR = 2,
    #[default]
    MACHINE = 3,
}

impl From<XLen> for MStatus_MPP {
    fn from(value: XLen) -> Self {
        match value {
            0b00 => MStatus_MPP::USER,
            0b01 => MStatus_MPP::SUPERVISOR,
            0b10 => MStatus_MPP::HYPERVISOR,
            0b11 => MStatus_MPP::MACHINE,

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(i64)]
#[derive(Default, Clone)]
pub enum MCause_Code {
    #[allow(clippy::identity_op)]
    INTERRUPT_RESERVED = 0 | SignedMXLen::MIN,
    INTERRUPT_SUPERVISOR_SW = 1 | SignedMXLen::MIN,
    INTERRUPT_RESERVED_2 = 2 | SignedMXLen::MIN,
    INTERRUPT_MACHINE_SW = 3 | SignedMXLen::MIN,

    INTERRUPT_RESERVED_4 = 4 | SignedMXLen::MIN,
    INTERRUPT_SUPERVISOR_TMR = 5 | SignedMXLen::MIN,
    INTERRUPT_RESERVED_6 = 6 | SignedMXLen::MIN,
    INTERRUPT_MACHINE_TMR = 7 | SignedMXLen::MIN,

    INTERRUPT_RESERVED_8 = 8 | SignedMXLen::MIN,
    INTERRUPT_SUPERVISOR_EXTERNAL = 9 | SignedMXLen::MIN,
    INTERRUPT_RESERVED_10 = 10 | SignedMXLen::MIN,
    INTERRUPT_MACHINE_EXTERNAL = 11 | SignedMXLen::MIN,

    INTERRUPT_RESERVED_12 = 12 | SignedMXLen::MIN,
    INTERRUPT_CNT_OVERFLOW = 13 | SignedMXLen::MIN,
    INTERRUPT_RESERVED_14 = 14 | SignedMXLen::MIN,
    INTERRUPT_PLATFORM_DESIGNATED = 15 | SignedMXLen::MIN,

    EXCEPTION_INSTR_ADDR_MISALIGNED = 0,
    EXCEPTION_INSTR_ACCESS_FAULT = 1,
    EXCEPTION_ILLEGAL_INSTR = 2,
    EXCEPTION_BREAKPOINT = 3,

    EXCEPTION_LOAD_ADDR_MISALIGNED = 4,
    EXCEPTION_LOAD_ACCESS_FAULT = 5,
    EXCEPTION_STR_AMO_ADDR_MISALGNED = 6,
    EXCEPTION_STR_AMO_ACCESS_FAULT = 7,

    EXCEPTION_ENV_CALL_FROM_U_MODE = 8,
    EXCEPTION_ENV_CALL_FROM_S_MODE = 9,
    EXCEPTION_RESERVED_10 = 10,
    EXCEPTION_ENV_CALL_FROM_M_MODE = 11,

    EXCEPTION_INSTR_PAGE_FAULT = 12,
    EXCEPTION_LOAD_PAGE_FAULT = 13,
    EXCEPTION_RESERVED_14 = 14,
    EXCEPTION_STR_AMO_PAGE_FAULT = 15,

    EXCEPTION_DOUBLE_TRAP = 16,
    EXCEPTION_RESERVED_17 = 17,
    EXCEPTION_SOFT_CHECK = 18,
    EXCEPTION_HW_ERROR = 19,

    EXCEPTION_RESERVED_20_23 = 20,

    #[default]
    EXCEPTION_CUSTOM_USE_DESIGNATED_24_31 = 24,

    EXCEPTION_RESERVED_32_47 = 32,

    EXCEPTION_CUSTOM_USE_DESIGNATED_48_63 = 48,

    EXCEPTION_RESERVED_64 = 64,
}
