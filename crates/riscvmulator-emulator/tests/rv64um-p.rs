mod common;
mod peripherals;

use common::run_test;

#[test]
fn div() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-div");
}

#[test]
fn divu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-divu");
}

#[test]
fn divuw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-divuw");
}

#[test]
fn divw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-divw");
}

#[test]
fn mul() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mul");
}

#[test]
fn mulh() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulh");
}

#[test]
fn mulhsu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulhsu");
}

#[test]
fn mulhu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulhu");
}

#[test]
fn mulw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulw");
}

#[test]
fn rem() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-rem");
}

#[test]
fn remu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-remu");
}

#[test]
fn remuw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-remuw");
}

#[test]
fn remw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-remw");
}
