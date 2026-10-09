// use std::error::Error;

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

// impl Error for GPRegisterName {};

// impl TryFrom<usize> for GPRegisterName {
//     fn try_from(value: usize) -> Result<Self, Self::Error> {
//         match value {
//             _ => Err(value),
//         }
//     }
// }
