mod common;
mod peripherals;

use common::run_test;

#[test]
fn xori() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-xori");
}

#[test]
fn xor() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-xor");
}

#[test]
fn subw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-subw");
}

#[test]
fn sub() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sub");
}

#[test]
fn srlw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srlw");
}

#[test]
fn srliw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srliw");
}

#[test]
fn srli() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srli");
}

#[test]
fn srl() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srl");
}

#[test]
fn sraw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sraw");
}

#[test]
fn sraiw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sraiw");
}

#[test]
fn srai() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srai");
}

#[test]
fn sra() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sra");
}

#[test]
fn sltu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sltu");
}

#[test]
fn sltiu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sltiu");
}

#[test]
fn slti() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slti");
}

#[test]
fn slt() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slt");
}

#[test]
fn sllw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sllw");
}

#[test]
fn slliw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slliw");
}

#[test]
fn slli() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slli");
}

#[test]
fn sll() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sll");
}

#[test]
fn st_ld() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-st_ld");
}

#[test]
fn sd() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sd");
}

#[test]
fn sw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sw");
}

#[test]
fn sh() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sh");
}

#[test]
fn sb() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sb");
}

#[test]
fn ori() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ori");
}

#[test]
fn or() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-or");
}

#[test]
fn ma_data() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ma_data");
}

#[test]
fn lui() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lui");
}

#[test]
fn ld_st() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ld_st");
}

#[test]
fn ld() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ld");
}

#[test]
fn lwu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lwu");
}

#[test]
fn lw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lw");
}

#[test]
fn lhu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lhu");
}

#[test]
fn lh() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lh");
}

#[test]
fn lbu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lbu");
}

#[test]
fn lb() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lb");
}

#[test]
fn jalr() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-jalr");
}

#[test]
fn jal() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-jal");
}

#[test]
fn fence_i() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-fence_i");
}

#[test]
fn simple() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-simple");
}

#[test]
fn bne() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bne");
}

#[test]
fn bltu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bltu");
}

#[test]
fn blt() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-blt");
}

#[test]
fn bgeu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bgeu");
}

#[test]
fn bge() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bge");
}

#[test]
fn beq() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-beq");
}

#[test]
fn auipc() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-auipc");
}

#[test]
fn andi() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-andi");
}

#[test]
fn and() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-and");
}

#[test]
fn addw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-addw");
}

#[test]
fn addiw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-addiw");
}

#[test]
fn addi() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-addi");
}

#[test]
fn add() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-add");
}
