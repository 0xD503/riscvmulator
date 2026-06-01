use crate::constants::arch::BusAddrType;
use crate::constants::arch::InstructionType;
use crate::soc::Bus;
use crate::soc::cpu::core::gp_registers::Register;
use crate::soc::cpu::core::registers::RegisterInterface;
use crate::soc::memory_bus::SystemBus;

use log::trace;

#[derive(Clone)]
pub struct FetchUnit {
    pub(crate) current_instruction: InstructionType,
}

impl FetchUnit {
    pub fn new(instr: InstructionType) -> Self {
        FetchUnit {
            current_instruction: instr,
        }
    }

    pub(in crate::soc::cpu) fn fetch(
        &mut self,
        bus: &SystemBus<BusAddrType>,
        pc: &Register,
    ) {
        trace!("Fetching instruction at addr: {:#018X?}", pc);

        self.current_instruction = bus.read(pc.read());

        trace!("Fetched instruction: {:#018X?}", self.current_instruction);
    }
}
