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

        let extended_rs1: i128 =
            decoder.source_register_1 as SignedXLen as i128;
        let extended_rs2: u128 = decoder.source_register_2 as u128;
        self.result =
            ((extended_rs1 * extended_rs2 as i128) >> XLen::BITS) as XLen;
    }

    pub(crate) fn instruction_multiply_high_unsigned(
        &mut self,
        decoder: &Decoder,
    ) {
        trace!("Executing MULHU...");

        let extended_rs1: u128 = decoder.source_register_1 as u128;
        let extended_rs2: u128 = decoder.source_register_2 as u128;
        self.result = ((extended_rs1 * extended_rs2) >> XLen::BITS) as XLen;
    }

    fn is_div_by_zero(&self, decoder: &Decoder) -> bool {
        if decoder.source_register_2 == 0 {
            true
        } else {
            false
        }
    }

    fn is_div_overflow(&self, decoder: &Decoder) -> bool {
        if decoder.source_register_2 as SignedXLen == -1
            && decoder.source_register_1 as SignedXLen == SignedXLen::MIN
        {
            true
        } else {
            false
        }
    }

    pub(crate) fn instruction_divide(&mut self, decoder: &Decoder) {
        trace!("Executing DIV...");

        if self.is_div_by_zero(decoder) {
            self.result = XLen::MAX;
        } else if self.is_div_overflow(decoder) {
            self.result = decoder.source_register_1;
        } else {
            self.result = (decoder.source_register_1 as SignedXLen
                / decoder.source_register_2 as SignedXLen)
                as XLen;
        }
    }

    pub(crate) fn instruction_divide_unsigned(&mut self, decoder: &Decoder) {
        trace!("Executing DIVU...");

        if self.is_div_by_zero(decoder) {
            self.result = XLen::MAX;
        } else {
            self.result = decoder.source_register_1 / decoder.source_register_2;
        }
    }

    pub(crate) fn instruction_remainder(&mut self, decoder: &Decoder) {
        trace!("Executing REM...");

        todo!();
    }

    pub(crate) fn instruction_remainder_unsigned(&mut self, decoder: &Decoder) {
        trace!("Executing REMU...");

        todo!();
    }
    // M32
    pub(crate) fn instruction_multiply_low_word(&mut self, decoder: &Decoder) {
        trace!("Executing MULW...");

        todo!();
    }
    pub(crate) fn instruction_divide_word(&mut self, decoder: &Decoder) {
        trace!("Executing DIVW...");

        todo!();
    }
    pub(crate) fn instruction_divide_unsigned_word(
        &mut self,
        decoder: &Decoder,
    ) {
        trace!("Executing DIVUW...");

        todo!();
    }
    pub(crate) fn instruction_remainder_word(&mut self, decoder: &Decoder) {
        trace!("Executing REMW...");

        todo!();
    }
    pub(crate) fn instruction_remainder_unsigned_word(
        &mut self,
        decoder: &Decoder,
    ) {
        trace!("Executing REMUW...");

        todo!();
    }
}
