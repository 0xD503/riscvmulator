pub mod misa {
    use crate::constants::arch::XLen;
    use crate::soc::hart::core::registers::{
        RawRegisterFileInterface, RegisterFileInterface,
    };
    use crate::soc::hart::extensions::zicsr::CSRFile;
    use crate::soc::hart::extensions::zicsr::constants::CSRRegisterName;
    use crate::soc::hart::privileged::constants::{
        MISA_EXTENSIONS_C_COMPRESSED, MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT,
        MXLen,
    };

    pub fn set_misa<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write(CSRRegisterName::misa as usize, value);
    }
    pub fn set_misa_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write_raw(CSRRegisterName::misa as usize, value);
    }

    pub fn read_misa<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
    ) -> MXLen {
        csrs.read(CSRRegisterName::misa as usize)
    }
    pub fn read_misa_raw<const REGS_NUM: usize>(
        csrs: &CSRFile<REGS_NUM>,
    ) -> MXLen {
        csrs.read_raw(CSRRegisterName::misa as usize)
    }

    pub fn is_m_enabled<const REGS_NUM: usize>(
        csrs: &CSRFile<REGS_NUM>,
    ) -> bool {
        let misa = read_misa_raw(csrs);

        (misa & MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT) != 0
    }

    pub fn is_c_enabled<const REGS_NUM: usize>(
        csrs: &CSRFile<REGS_NUM>,
    ) -> bool {
        let misa = read_misa_raw(csrs);

        (misa & MISA_EXTENSIONS_C_COMPRESSED) != 0
    }

    pub fn set_mstatus<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write(CSRRegisterName::mstatus as usize, value);
    }
    pub fn set_mstatus_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write_raw(CSRRegisterName::mstatus as usize, value);
    }

    pub fn set_mstatush<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write(CSRRegisterName::mstatush as usize, value);
    }
    pub fn set_mstatush_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        value: MXLen,
    ) {
        csrs.write_raw(CSRRegisterName::mstatush as usize, value);
    }

    pub fn set_mhartid<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        id: usize,
    ) {
        csrs.write(CSRRegisterName::mhartid as usize, id as XLen);
    }
    pub fn set_mhartid_raw<const REGS_NUM: usize>(
        csrs: &mut CSRFile<REGS_NUM>,
        id: usize,
    ) {
        csrs.write_raw(CSRRegisterName::mhartid as usize, id as XLen);
    }
}
