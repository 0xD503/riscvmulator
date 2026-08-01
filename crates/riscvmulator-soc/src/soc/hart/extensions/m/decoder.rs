use crate::soc::hart::core::{
    decoder::Decoder, executor::constants::ExecutableCommand,
};

impl Decoder {
    pub(crate) fn decode_m(&mut self) {
        match super::constants::OP_Funct3::from(self.funct3) {
            super::constants::OP_Funct3::MUL => {
                self.command_to_execute = ExecutableCommand::MUL;
            }
            super::constants::OP_Funct3::MULH => {
                self.command_to_execute = ExecutableCommand::MULH;
            }
            super::constants::OP_Funct3::MULHSU => {
                self.command_to_execute = ExecutableCommand::MULHSU;
            }
            super::constants::OP_Funct3::MULHU => {
                self.command_to_execute = ExecutableCommand::MULHU;
            }
            super::constants::OP_Funct3::DIV => {
                self.command_to_execute = ExecutableCommand::DIV;
            }
            super::constants::OP_Funct3::DIVU => {
                self.command_to_execute = ExecutableCommand::DIVU;
            }
            super::constants::OP_Funct3::REM => {
                self.command_to_execute = ExecutableCommand::REM;
            }
            super::constants::OP_Funct3::REMU => {
                self.command_to_execute = ExecutableCommand::REMU;
            }
        }
    }
    pub(crate) fn decode_m_32(&mut self) {
        match super::constants::OP_32_Funct3::from(self.funct3) {
            super::constants::OP_32_Funct3::MULW => {
                self.command_to_execute = ExecutableCommand::MULW;
            }
            super::constants::OP_32_Funct3::DIVW => {
                self.command_to_execute = ExecutableCommand::DIVW;
            }
            super::constants::OP_32_Funct3::DIVUW => {
                self.command_to_execute = ExecutableCommand::DIVUW;
            }
            super::constants::OP_32_Funct3::REMW => {
                self.command_to_execute = ExecutableCommand::REMW;
            }
            super::constants::OP_32_Funct3::REMUW => {
                self.command_to_execute = ExecutableCommand::REMUW;
            }
        }
    }
}
