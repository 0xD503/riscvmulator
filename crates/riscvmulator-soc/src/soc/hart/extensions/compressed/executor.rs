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
}
