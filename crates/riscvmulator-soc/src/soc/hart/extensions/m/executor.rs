use log::trace;

use crate::{
    constants::arch::{SignedXLen, XLen},
    soc::hart::core::{decoder::Decoder, executor::Executor},
};

impl Executor {
    pub(crate) fn instruction_multiply_low(&mut self, decoder: &Decoder) {
        trace!("Executing MUL...");

        self.result = decoder
            .source_register_1
            .wrapping_mul(decoder.source_register_2);
    }

    pub(crate) fn instruction_multiply_high(&mut self, decoder: &Decoder) {
        trace!("Executing MULH...");

        let extended_rs1: i128 =
            decoder.source_register_1 as SignedXLen as i128;
        let extended_rs2: i128 =
            decoder.source_register_2 as SignedXLen as i128;
        self.result = ((extended_rs1 * extended_rs2) >> XLen::BITS) as XLen;
    }

    pub(crate) fn instruction_multiply_high_signed_unsigned(
        &mut self,
        decoder: &Decoder,
    ) {
        trace!("Executing MULHSU...");

        todo!();
    }

    pub(crate) fn instruction_multiply_high_unsigned(
        &mut self,
        decoder: &Decoder,
    ) {
        trace!("Executing MULHU...");

        todo!();
    }

    pub(crate) fn instruction_divide(&mut self, decoder: &Decoder) {
        trace!("Executing DIV...");

        todo!();
    }

    pub(crate) fn instruction_divide_unsigned(&mut self, decoder: &Decoder) {
        trace!("Executing DIVU...");

        todo!();
    }

    pub(crate) fn instruction_remainder(&mut self, decoder: &Decoder) {
        trace!("Executing REM...");

        todo!();
    }

    pub(crate) fn instruction_remainder_unsigned(&mut self, decoder: &Decoder) {
        trace!("Executing REMU...");

        todo!();
    }
}
