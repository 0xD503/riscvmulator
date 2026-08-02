use log::trace;

use crate::config::SocCfg;
use crate::constants::arch::{RISCV_CSR_INITIAL_VALUE, XLen};
use crate::soc::hart::core::registers::{
    RawRegisterFileInterface, RawRegisterInterface, RegisterFileInterface,
    RegisterInterface,
};

pub mod constants;
use crate::soc::hart::privileged::constants::{
    MCause_Code, MISA_EXTENSIONS_A_ATOMIC, MISA_EXTENSIONS_C_COMPRESSED,
    MISA_EXTENSIONS_D_FP_DOUBLE, MISA_EXTENSIONS_F_FP_FLOAT,
    MISA_EXTENSIONS_I_BASE_INTEGER, MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT,
    MISA_MXL, MISA_MXL_OFFSET, MStatus_MPP, MXLen,
};
use crate::soc::hart::privileged::machine::misa::{
    set_mhartid_raw, set_misa_raw, set_mstatus_raw,
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub struct CSR {
    pub(in crate::soc::hart) value: XLen,
}

impl CSR {}

impl Default for CSR {
    fn default() -> Self {
        CSR {
            value: RISCV_CSR_INITIAL_VALUE,
        }
    }
}

#[derive(Clone)]
pub struct CSRFile<const REGS_NUM: usize> {
    pub(in crate::soc::hart) registers: [CSR; REGS_NUM],
}

impl<const REGS_NUM: usize> Default for CSRFile<REGS_NUM> {
    fn default() -> Self {
        Self {
            registers: [Default::default(); REGS_NUM],
        }
    }
}

impl RegisterInterface for CSR {
    fn read(&self) -> XLen {
        self.value
    }
    fn write(&mut self, value: XLen) {
        self.value = value;
    }

    fn reset(&mut self, value: XLen) {
        self.value = value;
    }
}

impl<const REGS_NUM: usize> RegisterFileInterface for CSRFile<REGS_NUM> {
    fn new() -> Self {
        Self {
            registers: [CSR {
                value: RISCV_CSR_INITIAL_VALUE,
            }; REGS_NUM],
        }
    }

    fn length() -> usize {
        REGS_NUM
    }

    fn read(&self, offset: usize) -> XLen {
        trace!("Reading CSR #{offset:#?}...");

        self.registers[offset].read()
    }

    fn write(&mut self, offset: usize, value: XLen) {
        trace!("Writing CSR #{offset:#?}...");

        self.registers[offset].write(value);
    }

    fn reset(&mut self) {
        for register in self.registers.iter_mut() {
            register.reset(RISCV_CSR_INITIAL_VALUE);
        }
    }
}

impl<const REGS_NUM: usize> CSRFile<REGS_NUM> {
    pub fn reset_and_bringup_hw_state(&mut self, cfg: &SocCfg, hartid: usize) {
        self.reset();

        // prepare and set `mstatus`
        // set priviliege mode to `M`
        let mstatus_value: MXLen = MStatus_MPP::MACHINE as MXLen;
        // set `MIE`, `MPRV`, `MBE` values to 0
        set_mstatus_raw(self, mstatus_value);

        // prepare and set `misa`
        let mut misa_value: MXLen;
        // MXL (Machine XLEN) field encodes the native base integer ISA width
        // TODO: currently only RV64I is supported
        misa_value = (MISA_MXL::XLEN_64 as MXLen) << MISA_MXL_OFFSET;
        if cfg.hart.extension_I {
            misa_value |= MISA_EXTENSIONS_I_BASE_INTEGER;
        }
        if cfg.hart.extension_M {
            misa_value |= MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT;
        }
        if cfg.hart.extension_A {
            misa_value |= MISA_EXTENSIONS_A_ATOMIC;
        }
        if cfg.hart.extension_F {
            misa_value |= MISA_EXTENSIONS_F_FP_FLOAT;
        }
        if cfg.hart.extension_D {
            misa_value |= MISA_EXTENSIONS_D_FP_DOUBLE;
        }
        if cfg.hart.extension_C {
            misa_value |= MISA_EXTENSIONS_C_COMPRESSED;
        }
        if cfg.hart.extension_Zifencei {
            // misa_value |= MISA_EXTENSIONS_M_MULTIPLY_DIVIDE_INT;
        }
        if cfg.hart.extension_Zicsr {
            // misa_value |= MISA_EXTENSIONS_Z;
        }
        set_misa_raw(self, misa_value);

        set_mhartid_raw(self, hartid);
    }
}

impl RawRegisterInterface for CSR {
    fn read_raw(&self) -> XLen {
        self.value
    }
    fn write_raw(&mut self, value: XLen) {
        self.value = value;
    }
}

impl<const REGS_NUM: usize> RawRegisterFileInterface for CSRFile<REGS_NUM> {
    fn read_raw(&self, offset: usize) -> XLen {
        self.registers[offset].read()
    }

    fn write_raw(&mut self, offset: usize, value: XLen) {
        self.registers[offset].write(value);
    }
}

#[derive(Default, Clone)]
pub struct DecoderZicsrContext {
    pub(crate) need_to_read_csr: bool,
    pub(crate) need_to_write_csr: bool,
    pub(crate) need_to_return_from_trap: bool,
    pub(crate) need_to_call_trap: bool,
    pub(crate) uimm: XLen,
}

#[derive(Default, Clone)]
pub struct ExecutorZicsrContext {
    pub(crate) csr_value: XLen,
    pub(crate) trap_cause: MCause_Code,
    pub(crate) trap_vector: XLen,
    pub(crate) epc: XLen, // exception program counter
}
