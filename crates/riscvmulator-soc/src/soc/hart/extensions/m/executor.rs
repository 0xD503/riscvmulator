use log::trace;

use crate::soc::hart::core::{decoder::Decoder, executor::Executor};

impl Executor {
    pub(crate) fn instruction_multiply_low(&mut self, decoder: &Decoder) {
        trace!("Executing MUL...");

        todo!();
    }

    pub(crate) fn instruction_multiply_high(&mut self, decoder: &Decoder) {
        trace!("Executing MULH...");

        todo!();
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
