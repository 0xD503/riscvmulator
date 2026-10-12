use log::debug;

#[cfg(feature = "print-instructions")]
use crate::print_instructions::print_full_instruction;
#[cfg(feature = "print-instructions")]
use crate::soc::hart::core::gp_registers::constants::GPRegisterName;
use crate::{
    constants::arch::{SignedXLen, SignedXLen32, XLen, XLen32},
    soc::{
        hart::core::{decoder::Decoder, executor::Executor},
        memory::MemoryValueType,
    },
};

impl Executor {
    pub(crate) fn instruction_multiply_low(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "mul",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value,
            decoder.source_register_2_value
        );

        self.result = decoder
            .source_register_1_value
            .wrapping_mul(decoder.source_register_2_value);
    }

    pub(crate) fn instruction_multiply_high(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "mulh",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as SignedXLen as i128,
            decoder.source_register_2_value as SignedXLen as i128
        );

        let extended_rs1: i128 =
            decoder.source_register_1_value as SignedXLen as i128;
        let extended_rs2: i128 =
            decoder.source_register_2_value as SignedXLen as i128;
        self.result = ((extended_rs1 * extended_rs2) >> XLen::BITS) as XLen;
    }

    pub(crate) fn instruction_multiply_high_signed_unsigned(
        &mut self,
        decoder: &Decoder,
    ) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "mulhsu",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as SignedXLen as i128,
            decoder.source_register_2_value as SignedXLen as i128
        );

        let extended_rs1: i128 =
            decoder.source_register_1_value as SignedXLen as i128;
        let extended_rs2: u128 = decoder.source_register_2_value as u128;
        self.result =
            ((extended_rs1 * extended_rs2 as i128) >> XLen::BITS) as XLen;
    }

    pub(crate) fn instruction_multiply_high_unsigned(
        &mut self,
        decoder: &Decoder,
    ) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "mulhu",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as i128,
            decoder.source_register_2_value as i128
        );

        let extended_rs1: u128 = decoder.source_register_1_value as u128;
        let extended_rs2: u128 = decoder.source_register_2_value as u128;
        self.result = ((extended_rs1 * extended_rs2) >> XLen::BITS) as XLen;
    }

    fn is_div_by_zero(&self, decoder: &Decoder) -> bool {
        decoder.source_register_2_value == 0
    }

    fn is_div_overflow(&self, decoder: &Decoder) -> bool {
        decoder.source_register_2_value as SignedXLen == -1
            && decoder.source_register_1_value as SignedXLen == SignedXLen::MIN
    }

    fn is_div_by_zero_32(&self, decoder: &Decoder) -> bool {
        decoder.source_register_2_value as XLen32 == 0
    }

    fn is_div_overflow_32(&self, decoder: &Decoder) -> bool {
        decoder.source_register_2_value as XLen32 as SignedXLen32 == -1
            && decoder.source_register_1_value as XLen32 as SignedXLen32
                == SignedXLen32::MIN
    }

    pub(crate) fn instruction_divide(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "div",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as SignedXLen,
            decoder.source_register_2_value as SignedXLen
        );

        if self.is_div_by_zero(decoder) {
            self.result = XLen::MAX;
        } else if self.is_div_overflow(decoder) {
            self.result = decoder.source_register_1_value;
        } else {
            self.result = (decoder.source_register_1_value as SignedXLen
                / decoder.source_register_2_value as SignedXLen)
                as XLen;
        }
    }

    pub(crate) fn instruction_divide_unsigned(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "divu",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value,
            decoder.source_register_2_value
        );

        if self.is_div_by_zero(decoder) {
            self.result = XLen::MAX;
        } else {
            self.result = decoder.source_register_1_value
                / decoder.source_register_2_value;
        }
    }

    pub(crate) fn instruction_remainder(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "rem",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as SignedXLen,
            decoder.source_register_2_value as SignedXLen
        );

        if self.is_div_by_zero(decoder) {
            self.result = decoder.source_register_1_value;
        } else if self.is_div_overflow(decoder) {
            self.result = 0;
        } else {
            self.result = (decoder.source_register_1_value as SignedXLen
                % decoder.source_register_2_value as SignedXLen)
                as XLen;
        }
    }

    pub(crate) fn instruction_remainder_unsigned(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op,
            "remu",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value,
            decoder.source_register_2_value
        );

        if self.is_div_by_zero(decoder) {
            self.result = decoder.source_register_1_value;
        } else {
            self.result = decoder.source_register_1_value
                % decoder.source_register_2_value;
        }
    }
    // M32
    /// MULW is an RV64 instruction that multiplies the lower 32 bits of the
    /// source registers, placing the sign extension of the lower 32 bits of the
    /// result into the destination register
    pub(crate) fn instruction_multiply_low_word(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op_32,
            "mulw",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as XLen32,
            decoder.source_register_2_value as XLen32
        );

        // self.result = (((decoder.source_register_1 & XLen32::MAX as XLen)
        //     * (decoder.source_register_2 & XLen32::MAX as XLen))
        //     & XLen32::MAX as XLen)
        //     .sign_extend();
        self.result = (decoder.source_register_1_value as XLen32)
            .wrapping_mul(decoder.source_register_2_value as XLen32)
            .sign_extend();
    }
    pub(crate) fn instruction_divide_word(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op_32,
            "divw",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as SignedXLen32,
            decoder.source_register_2_value as SignedXLen32
        );

        if self.is_div_by_zero_32(decoder) {
            self.result = -1 as SignedXLen as XLen;
        } else if self.is_div_overflow_32(decoder) {
            self.result =
                (decoder.source_register_1_value as XLen32).sign_extend();
        } else {
            self.result = ((decoder.source_register_1_value as SignedXLen32)
                .wrapping_div(decoder.source_register_2_value as SignedXLen32))
                as SignedXLen as XLen;
        }
    }
    pub(crate) fn instruction_divide_unsigned_word(
        &mut self,
        decoder: &Decoder,
    ) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op_32,
            "divuw",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as XLen32,
            decoder.source_register_2_value as XLen32
        );

        if self.is_div_by_zero_32(decoder) {
            self.result = XLen::MAX;
        } else {
            self.result = ((decoder.source_register_1_value as XLen32)
                .wrapping_div(decoder.source_register_2_value as XLen32))
            .sign_extend();
        }
    }
    pub(crate) fn instruction_remainder_word(&mut self, decoder: &Decoder) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op_32,
            "remw",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as SignedXLen32,
            decoder.source_register_2_value as SignedXLen32
        );

        if self.is_div_by_zero_32(decoder) {
            self.result =
                (decoder.source_register_1_value as XLen32).sign_extend();
        } else if self.is_div_overflow_32(decoder) {
            self.result = 0;
        } else {
            self.result = ((decoder.source_register_1_value as SignedXLen32)
                .wrapping_rem(decoder.source_register_2_value as SignedXLen32))
                as SignedXLen as XLen;
        }
    }
    pub(crate) fn instruction_remainder_unsigned_word(
        &mut self,
        decoder: &Decoder,
    ) {
        #[cfg(feature = "print-instructions")]
        print_full_instruction!(
            op_32,
            "remuw",
            decoder.destination_register_offset,
            decoder.source_register_1_offset,
            decoder.source_register_2_offset,
            decoder.source_register_1_value as XLen32,
            decoder.source_register_2_value as XLen32
        );

        if self.is_div_by_zero_32(decoder) {
            self.result =
                (decoder.source_register_1_value as XLen32).sign_extend();
        } else {
            self.result = ((decoder.source_register_1_value as XLen32)
                % (decoder.source_register_2_value as XLen32))
                .sign_extend();
        }
    }
}
