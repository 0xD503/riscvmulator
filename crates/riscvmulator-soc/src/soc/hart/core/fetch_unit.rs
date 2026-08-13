use crate::constants::arch::{BusAddrType, InstructionType, XLen};
use crate::constants::soc::INITIAL_INSTRUCTION;
use crate::soc::Bus;
use crate::soc::hart::core::decoder::constants::{
    InstructionSize, OPCODE_QUADRANT_MASK, OpcodeQuadrant,
};
use crate::soc::hart::core::gp_registers::Register;
use crate::soc::hart::core::registers::RegisterInterface;
use crate::soc::hart::extensions::c::constants::InstructionTypeCompressed;
use crate::soc::memory_bus::SystemBus;

use log::trace;

#[derive(Clone)]
pub struct FetchUnit {
    pub(in crate::soc) instruction_size: InstructionSize,
    pub(in crate::soc) current_instruction: InstructionType,
    pub(in crate::soc) current_instruction_compressed:
        InstructionTypeCompressed,
}

impl FetchUnit {
    pub fn new(instr: InstructionType, instr_size: InstructionSize) -> Self {
        FetchUnit {
            instruction_size: instr_size,
            current_instruction: instr,
            current_instruction_compressed: instr as InstructionTypeCompressed,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::get_reset_instance();
    }

    fn get_reset_instance() -> Self {
        Self {
            instruction_size: InstructionSize::I32,
            current_instruction: INITIAL_INSTRUCTION,
            current_instruction_compressed: INITIAL_INSTRUCTION
                as InstructionTypeCompressed,
        }
    }

    pub(crate) fn extract_quadrant(
        &self,
        instruction: InstructionType,
    ) -> OpcodeQuadrant {
        OpcodeQuadrant::from(instruction & OPCODE_QUADRANT_MASK)
    }

    pub(in crate::soc::hart) fn fetch(
        &mut self,
        bus: &SystemBus<BusAddrType>,
        pc: &Register,
    ) {
        trace!("Fetching instruction at addr: {:#018X?}", pc.read());

        // //
        // // first way
        // //
        // // we may have a compressed instruction, so we need to check quadrant
        // self.current_instruction = bus.instruction_read(pc.read());
        // let quadrant = self.extract_quadrant(self.current_instruction);
        // match quadrant {
        //     // uncompressed instruction set
        //     OpcodeQuadrant::Quadrant11 => {
        //         self.instruction_size = InstructionSize::I32;
        //         trace!(
        //             "Fetched instruction: {:#010X?}",
        //             self.current_instruction
        //         );
        //     }
        //     // compressed instruction set
        //     _ => {
        //         self.current_instruction_compressed =
        //             self.current_instruction as InstructionTypeCompressed;
        //         self.instruction_size = InstructionSize::I16;
        //         trace!(
        //             "Fetched instruction: {:#06X?}",
        //             self.current_instruction_compressed
        //         );
        //     }
        // }

        //
        // second way
        //
        // we may have a compressed instruction, so we need to check quadrant
        let mut current_instruction_dest =
            [0u8; size_of::<InstructionTypeCompressed>()];

        bus.instruction_read(pc.read(), &mut current_instruction_dest)
            .unwrap_or_else(|err| {
                todo!("Is it even possible???: {err}");
            });
        let current_instruction =
            InstructionTypeCompressed::from_le_bytes(current_instruction_dest);
        let quadrant =
            self.extract_quadrant(current_instruction as InstructionType);
        match quadrant {
            OpcodeQuadrant::Quadrant11 => {
                // non-compressed instruction, read more batches
                // save first batch (lower)
                self.current_instruction =
                    current_instruction as InstructionType;
                // read high part of instruction
                bus.instruction_read(
                    pc.read() + InstructionSize::I16 as XLen,
                    &mut current_instruction_dest,
                )
                .unwrap_or_else(|err| {
                    todo!("Is it even possible???: {err}");
                });
                self.current_instruction |=
                    (InstructionTypeCompressed::from_le_bytes(
                        current_instruction_dest,
                    ) as InstructionType)
                        << InstructionTypeCompressed::BITS;
                self.instruction_size = InstructionSize::I32;
                trace!(
                    "Fetched instruction: {:#010X?}",
                    self.current_instruction
                );
            }
            // compressed instruction set
            OpcodeQuadrant::Quadrant00
            | OpcodeQuadrant::Quadrant01
            | OpcodeQuadrant::Quadrant10 => {
                self.current_instruction_compressed = current_instruction;
                self.instruction_size = InstructionSize::I16;
                trace!(
                    "Fetched instruction: {:#06X?}",
                    self.current_instruction_compressed
                );
            }
        }
    }
}
