// use crate::

use riscvmulator::elf_loader::load_elf;
use riscvmulator::{SoC, init};

fn main() {
    init();

    let mut soc: SoC = SoC::new();

    soc.reset();

    load_elf("3rd_party/riscv-tests/isa/rv64ui-p-add".into(), &mut soc);

    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
