mod common;
mod peripherals;

use common::run_test;

const TEST_CONFIG_PATH: &str = "rv64im_single_hart.toml";

riscv_test!("rv64um-p", div);
riscv_test!("rv64um-p", divu);
riscv_test!("rv64um-p", divuw);
riscv_test!("rv64um-p", divw);
riscv_test!("rv64um-p", mul);
riscv_test!("rv64um-p", mulh);
riscv_test!("rv64um-p", mulhsu);
riscv_test!("rv64um-p", mulhu);
riscv_test!("rv64um-p", mulw);
riscv_test!("rv64um-p", rem);
riscv_test!("rv64um-p", remu);
riscv_test!("rv64um-p", remuw);
riscv_test!("rv64um-p", remw);
