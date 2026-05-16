// use crate::

use riscvmulator::elf_loader::load_elf;
use riscvmulator::{SoC, init};

fn main() {
    let args = init();

    let mut soc: SoC = SoC::new();

    soc.reset();

    load_elf(args.input_file.into(), &mut soc);

    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
