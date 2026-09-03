#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone, Debug)]
pub enum ExecutableCommand {
    LB = 0, // load byte
    LH = 1, // load halfword
    LW = 2, // load word

    LD = 3,  // load double word
    LBU = 4, // load byte (unsigned)
    LHU = 5, // load halfword (unsigned)
    LWU = 6, // load word (unsigned)

    ADD = 7,   // add registers
    SUB = 8,   // subtract registers
    SLL = 9,   // shift left logical registers
    SLT = 10,  // set if register less then register
    SLTU = 11, // set if register less then register (unsigned)
    XOR = 12,  // XOR with register
    SRL = 13,  // shift right logical register
    SRA = 14,  // shift right arithmetic register
    OR = 15,   // OR with register
    AND = 16,  // AND with register

    ADDW = 17, // add word (register)
    SUBW = 18, // subtract word (register)
    SLLW = 19, // shift left logical register word
    SRLW = 20, // shift right logical register word
    SRAW = 21, // shift right arithmetic register word

    #[default]
    ADDI = 22, // add immediate
    SLLI = 23,  // shift left logical immediate
    SLTI = 24,  // set if register less then immediate
    SLTIU = 25, // set if register less then immediate (unsigned)
    XORI = 26,  // XOR with immediate
    SRLI = 27,  // shift right logical immediate
    SRAI = 28,  // shift right arithmetic immediate
    ORI = 29,   // OR with immediate
    ANDI = 30,  // AND with immediate
    JALR = 31,  // jump and link register

    ADDIW = 32, // add immediate word
    SLLIW = 33, // shift left logical immediate word
    SRLIW = 34, // shift right logical immediate word
    SRAIW = 35, // shift right arithmetic immediate word

    SB = 36, // store byte
    SH = 37, // store halfword
    SW = 38, // store word

    SD = 39, // store double word

    AUIPC = 40, // add upper intermideate to PC
    LUI = 41,   // load upper intermideate

    BEQ = 42,  // branch if equal
    BNE = 43,  // branch if not equal
    BLT = 44,  // branch if less than
    BGE = 45,  // branch if greater or equal than
    BLTU = 46, // branch if less than (unsigned)
    BGEU = 47, // branch if greater or equal than (unsigned)

    JAL = 48, // jump and link

    ECALL = 49,  // make service request to the execution environment
    EBREAK = 50, // return control to debugging environment

    FENCE = 51,   // order I/O and memory accesses
    FENCE_I = 52, // Zifencei: order I/O and memory accesses

    URET = 53, // return from user exception
    SRET = 54, // return from supervisor exception
    MRET = 55, // return from machine exception

    CSRRW = 56,  // CSR read/write
    CSRRS = 57,  // CSR read/set
    CSRRC = 58,  // CSR read/clear
    CSRRWI = 59, // CSR read/write immediate
    CSRRSI = 60, // CSR read/set immediate
    CSRRCI = 61, // CSR read/clear immediate

    /// `M` extension instrurctions
    MUL = 62,
    MULH = 63,
    MULHSU = 64,
    MULHU = 65,
    DIV = 66,
    DIVU = 67,
    REM = 68,
    REMU = 69,
    // M32
    MULW = 70,
    DIVW = 71,
    DIVUW = 72,
    REMW = 73,
    REMUW = 74,

    /// `C` extension instructions
    C_ADDI4SPN = 75,
}
