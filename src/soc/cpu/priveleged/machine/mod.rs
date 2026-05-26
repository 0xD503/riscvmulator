// TODO: disallow dead code
#![allow(dead_code)]

use log::error;

use crate::constants::arch::XLen;

pub type MXLen = u64;
pub type SignedMXLen = i64;
pub type UnsignedMXLen = u64;

pub const M_REGISTER_NOT_IMPLEMENTED: MXLen = 0;

pub const MISA_EXTENSIONS_A_ATOMIC: MXLen = 0x00000001;
pub const MISA_EXTENSIONS_B_B: MXLen = 0x00000002;
pub const MISA_EXTENSIONS_C_COMPRESSED: MXLen = 0x00000004;
pub const MISA_EXTENSIONS_D_FP_DOUBLE: MXLen = 0x00000008;
pub const MISA_EXTENSIONS_E_EMBEDDED: MXLen = 0x00000010;
pub const MISA_EXTENSIONS_F_FP_FLOAT: MXLen = 0x00000020;
pub const MISA_EXTENSIONS_G_RESERVED: MXLen = 0x00000040;
pub const MISA_EXTENSIONS_H_HYPERVISOR: MXLen = 0x00000080;
pub const MISA_EXTENSIONS_I_BASE_INTEGER: MXLen = 0x00000100;
pub const MISA_EXTENSIONS_J_RESERVED: MXLen = 0x00000200;
pub const MISA_EXTENSIONS_K_RESERVED: MXLen = 0x00000400;
pub const MISA_EXTENSIONS_L_RESERVED: MXLen = 0x00000800;
pub const MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT: MXLen = 0x00001000;
pub const MISA_EXTENSIONS_N_USER_LEVEL_INTERRUPTS: MXLen = 0x00002000;
pub const MISA_EXTENSIONS_O_RESERVED: MXLen = 0x00004000;
pub const MISA_EXTENSIONS_P_PACKED_SIMD: MXLen = 0x00008000;
pub const MISA_EXTENSIONS_Q_QUAD_PRECISION_FP: MXLen = 0x00010000;
pub const MISA_EXTENSIONS_R_RESERVED: MXLen = 0x00020000;
pub const MISA_EXTENSIONS_S_SUPERVISOR_MODE: MXLen = 0x00040000;
pub const MISA_EXTENSIONS_T_RESERVED: MXLen = 0x00080000;
pub const MISA_EXTENSIONS_U_USER_MODE: MXLen = 0x00100000;
pub const MISA_EXTENSIONS_V_VECTOR: MXLen = 0x00200000;
pub const MISA_EXTENSIONS_W_RESERVED: MXLen = 0x00400000;
pub const MISA_EXTENSIONS_X_NON_STANDART: MXLen = 0x00800000;
pub const MISA_EXTENSIONS_Y_RESERVED: MXLen = 0x01000000;
pub const MISA_EXTENSIONS_Z_RESERVED: MXLen = 0x02000000;

pub const MISA_MXL_OFFSET: usize = size_of::<MXLen>() - 2;

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
