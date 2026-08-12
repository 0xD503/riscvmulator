use crate::constants::arch::RISCV_NUM_OF_REGISTERS;
use crate::soc::hart::core::{
    ArchState, decoder::Decoder, fetch_unit::FetchUnit,
    gp_registers::RegisterFile,
};

impl Decoder {
    pub(crate) fn decode_c(
        &mut self,
        arch_state: &ArchState,
        fetch_unit: &FetchUnit,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) {
        // self.opcode = self.extract_opcode_compressed(fetch_unit);
        // let quadrant = fetch_unit.extract_quadrant(instruction)

        // match From::from(self.opcode) {
        //     OpcodeToInstructionFormat::
        // }

        todo!("TODO: compressed ISA decoder");
    }

    fn extract_opcode_compressed(&self, fetch_unit: &FetchUnit) {
        //
    }

    fn decode_cr(&mut self) {
        //
    }

    fn decode_ci(&mut self) {
        //
    }

    fn decode_css(&mut self) {
        //
    }

    fn decode_ciw(&mut self) {
        //
    }

    fn decode_cl(&mut self) {
        //
    }

    fn decode_cs(&mut self) {
        //
    }

    fn decode_ca(&mut self) {
        //
    }

    fn decode_cb(&mut self) {
        //
    }

    fn decode_cj(&mut self) {
        //
    }
}
