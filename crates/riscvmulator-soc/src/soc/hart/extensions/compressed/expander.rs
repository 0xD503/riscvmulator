use crate::soc::hart::{
    core::{
        ArchState, decoder::constants::InstructionSize, fetch_unit::FetchUnit,
    },
    privileged::machine::misa,
};

#[derive(Clone)]
pub struct Expander {
    //
}

impl Expander {
    pub fn new() -> Self {
        Expander {}
    }

    pub fn expand(&mut self, arch_state: &ArchState, fetch_unit: &FetchUnit) {
        if fetch_unit.instruction_size == InstructionSize::I16 {
            // compressed ISA
            if misa::is_c_enabled(&arch_state.control_status_registers) {
                // let decompressed_instruction =
                //     self.expand_compressed(fetch_unit);
                // self.decode_i_32(arch_state, decompressed_instruction);
            } else {
                todo!("Illegal instruction trap");
            }
        }
    }
}
