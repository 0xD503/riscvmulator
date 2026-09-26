use log::debug;

use crate::soc::hart::core::{decoder::Decoder, executor::Executor};

impl Executor {
    // CIW
    pub(crate) fn instruction_add_immediate_to_stack_pointer_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.ADDI4SPN...");

        //

        todo!("TODO: unimplemented");
    }

    // CL
    pub(crate) fn instruction_load_double_precision_fp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FLD...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_word_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.LW...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_single_precision_fp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FLW...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_double_word_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.LD...");

        //

        todo!("TODO: unimplemented");
    }

    // CS
    pub(crate) fn instruction_store_double_precision_fp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FSD...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_store_word_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.SW...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_store_single_precision_fp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FSW...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_store_double_word_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.SD...");

        //

        todo!("TODO: unimplemented");
    }

    // CA
    pub(crate) fn instruction_and_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.AND...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_or_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.OR...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_xor_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.XOR...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_sub_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.SUB...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_add_word_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.ADDW...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_sub_word_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.SUBW...");

        //

        todo!("TODO: unimplemented");
    }

    // CB
    /// C.BEQZ performs conditional control transfers. The offset is
    /// sign-extended and added to the pc to form the branch target address. It
    /// can therefore target a +/-256 B range. C.BEQZ takes the branch if the
    /// value in register rs1' is zero. It expands to beq rs1', x0, offset
    pub(crate) fn instruction_branch_if_equals_zero_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.BEQZ...");

        //

        todo!("TODO: unimplemented");
    }

    /// C.BNEZ is defined analogously to C.BQEZ, but it takes the branch if rs1'
    /// contains a nonzero value. It expands to bne rs1', x0, offset.
    pub(crate) fn instruction_branch_if_not_equals_zero_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.BNEZ...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_logical_shift_right_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.SRLI...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_arithmetic_shift_right_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.SRAI...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_and_immediate_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.ANDI...");

        //

        todo!("TODO: unimplemented");
    }

    // CJ
    pub(crate) fn instruction_jump_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.J...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_jal_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.JAL...");

        //

        todo!("TODO: unimplemented");
    }

    // CR
    pub(crate) fn instruction_jump_to_addr_in_register_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.JR...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_jalr_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.JALR...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_move_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.MV...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_add_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.ADD...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_ebreak_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.EBREAK...");

        //

        todo!("TODO: unimplemented");
    }

    // CI
    pub(crate) fn instruction_load_word_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.LWSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_double_word_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.LDSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_single_precision_fp_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FLWSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_double_precision_fp_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FLDSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_immediate_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.LI...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_load_upper_immediate_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.LUI...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_add_immediate_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.ADDI...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_add_immediate_to_word_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.ADDIW...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_add_immediate_16_to_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.ADDI16SP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_logical_shift_left_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.SLLI...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_nop_compressed(&self, decoder: &Decoder) {
        debug!("Executing C.NOP...");

        //

        todo!("TODO: unimplemented");
    }

    // CSS
    pub(crate) fn instruction_store_word_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.SWSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_store_double_word_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.SDSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_store_single_precision_fp_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FSWSP...");

        //

        todo!("TODO: unimplemented");
    }

    pub(crate) fn instruction_store_double_precision_fp_sp_compressed(
        &self,
        decoder: &Decoder,
    ) {
        debug!("Executing C.FSDSP...");

        //

        todo!("TODO: unimplemented");
    }
}
