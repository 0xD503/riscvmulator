// use std::error::Error;

use std::{error::Error, fmt::Display};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum GPRegisterName {
    x0_zero,  // always zero
    x1_ra,    // return address
    x2_sp,    // stack pointer
    x3_gp,    // global pointer
    x4_tp,    // thread pointer
    x5_t0,    // temporary register 0
    x6_t1,    // temporary register 1
    x7_t2,    // temporary register 2
    x8_s0_fp, // saved register 0 / frame pointer
    x9_s1,    // saved register 1
    x10_a0,   // function argument 0 / return value 0
    x11_a1,   // function argument 1 / return value 1
    x12_a2,   // function argument 2
    x13_a3,   // function argument 3
    x14_a4,   // function argument 4
    x15_a5,   // function argument 5
    x16_a6,   // function argument 6
    x17_a7,   // function argument 7
    x18_s2,   // saved register 2
    x19_s3,   // saved register 3
    x20_s4,   // saved register 4
    x21_s5,   // saved register 5
    x22_s6,   // saved register 6
    x23_s7,   // saved register 7
    x24_s8,   // saved register 8
    x25_s9,   // saved register 9
    x26_s10,  // saved register 10
    x27_s11,  // saved register 11
    x28_t3,   // temporary register 3
    x29_t4,   // temporary register 4
    x30_t5,   // temporary register 5
    x31_t6,   // temporary register 6
}

impl TryFrom<usize> for GPRegisterName {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GPRegisterName::x0_zero),
            1 => Ok(GPRegisterName::x1_ra),
            2 => Ok(GPRegisterName::x2_sp),
            3 => Ok(GPRegisterName::x3_gp),
            4 => Ok(GPRegisterName::x4_tp),
            5 => Ok(GPRegisterName::x5_t0),
            6 => Ok(GPRegisterName::x6_t1),
            7 => Ok(GPRegisterName::x7_t2),
            8 => Ok(GPRegisterName::x8_s0_fp),
            9 => Ok(GPRegisterName::x9_s1),
            10 => Ok(GPRegisterName::x10_a0),
            11 => Ok(GPRegisterName::x11_a1),
            12 => Ok(GPRegisterName::x12_a2),
            13 => Ok(GPRegisterName::x13_a3),
            14 => Ok(GPRegisterName::x14_a4),
            15 => Ok(GPRegisterName::x15_a5),
            16 => Ok(GPRegisterName::x16_a6),
            17 => Ok(GPRegisterName::x17_a7),
            18 => Ok(GPRegisterName::x18_s2),
            19 => Ok(GPRegisterName::x19_s3),
            20 => Ok(GPRegisterName::x20_s4),
            21 => Ok(GPRegisterName::x21_s5),
            22 => Ok(GPRegisterName::x22_s6),
            23 => Ok(GPRegisterName::x23_s7),
            24 => Ok(GPRegisterName::x24_s8),
            25 => Ok(GPRegisterName::x25_s9),
            26 => Ok(GPRegisterName::x26_s10),
            27 => Ok(GPRegisterName::x27_s11),
            28 => Ok(GPRegisterName::x28_t3),
            29 => Ok(GPRegisterName::x29_t4),
            30 => Ok(GPRegisterName::x30_t5),
            31 => Ok(GPRegisterName::x31_t6),

            _ => Err("unknown register offset"),
        }
    }
}

impl Display for GPRegisterName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let register_name_string;
        match self {
            GPRegisterName::x0_zero => register_name_string = "x0_zero",
            GPRegisterName::x1_ra => register_name_string = "x1_ra",
            GPRegisterName::x2_sp => register_name_string = "x2_sp",
            GPRegisterName::x3_gp => register_name_string = "x3_gp",
            GPRegisterName::x4_tp => register_name_string = "x4_tp",
            GPRegisterName::x5_t0 => register_name_string = "x5_t0",
            GPRegisterName::x6_t1 => register_name_string = "x6_t1",
            GPRegisterName::x7_t2 => register_name_string = "x7_t2",
            GPRegisterName::x8_s0_fp => register_name_string = "x8_s0_fp",
            GPRegisterName::x9_s1 => register_name_string = "x9_s1",
            GPRegisterName::x10_a0 => register_name_string = "x10_a0",
            GPRegisterName::x11_a1 => register_name_string = "x11_a1",
            GPRegisterName::x12_a2 => register_name_string = "x12_a2",
            GPRegisterName::x13_a3 => register_name_string = "x13_a3",
            GPRegisterName::x14_a4 => register_name_string = "x14_a4",
            GPRegisterName::x15_a5 => register_name_string = "x15_a5",
            GPRegisterName::x16_a6 => register_name_string = "x16_a6",
            GPRegisterName::x17_a7 => register_name_string = "x17_a7",
            GPRegisterName::x18_s2 => register_name_string = "x18_s2",
            GPRegisterName::x19_s3 => register_name_string = "x19_s3",
            GPRegisterName::x20_s4 => register_name_string = "x20_s4",
            GPRegisterName::x21_s5 => register_name_string = "x21_s5",
            GPRegisterName::x22_s6 => register_name_string = "x22_s6",
            GPRegisterName::x23_s7 => register_name_string = "x23_s7",
            GPRegisterName::x24_s8 => register_name_string = "x24_s8",
            GPRegisterName::x25_s9 => register_name_string = "x25_s9",
            GPRegisterName::x26_s10 => register_name_string = "x26_s10",
            GPRegisterName::x27_s11 => register_name_string = "x27_s11",
            GPRegisterName::x28_t3 => register_name_string = "x28_t3",
            GPRegisterName::x29_t4 => register_name_string = "x29_t4",
            GPRegisterName::x30_t5 => register_name_string = "x30_t5",
            GPRegisterName::x31_t6 => register_name_string = "x31_t6",
        }

        write!(formatter, "{register_name_string}")
    }
}

// impl Error for GPRegisterName {
//     // f
// }
