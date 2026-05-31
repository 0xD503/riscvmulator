use crate::constants::arch::InstructionType;

#[derive(Clone)]
pub struct FetchUnit {
    current_instruction: InstructionType,
}

impl FetchUnit {
    pub fn new(instr: InstructionType) -> Self {
        FetchUnit {
            current_instruction: instr,
        }
    }

    pub fn current_instruction(&self) -> InstructionType {
        self.current_instruction
    }

    pub fn set_current_instruction(&mut self, value: InstructionType) {
        self.current_instruction = value;
    }
}
