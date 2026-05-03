#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone, Debug)]
pub enum ExecutableCommand {
    #[default]
    LB, // load byte
    LH, // load halfword
    LW, // load word

    LD,  // load double word
    LBU, // load byte (unsigned)
    LHU, // load halfword (unsigned)
    LWU, // load word (unsigned)

    ADD,  // add registers
    SUB,  // subtract registers
    SLL,  // shift left logical registers
    SLT,  // set if register less then register
    SLTU, // set if register less then register (unsigned)
    XOR,  // XOR with register
    SRL,  // shift right logical register
    SRA,  // shift right arithmetic register
    OR,   // OR with register
    AND,  // AND with register

    ADDW, // add word (register)
    SUBW, // subtract word (register)
    SLLW, // shift left logical register word
    SRLW, // shift right logical register word
    SRAW, // shift right arithmetic register word

    ADDI,  // add immediate
    SLLI,  // shift left logical immediate
    SLTI,  // set if register less then immediate
    SLTIU, // set if register less then immediate (unsigned)
    XORI,  // XOR with immediate
    SRLI,  // shift right logical immediate
    SRAI,  // shift right arithmetic immediate
    ORI,   // OR with immediate
    ANDI,  // AND with immediate
    JALR,  // jump and link register

    ADDIW, // add immediate word
    SLLIW, // shift left logical immediate word
    SRLIW, // shift right logical immediate word
    SRAIW, // shift right arithmetic immediate word

    SB, // store byte
    SH, // store halfword
    SW, // store word

    SD, // store double word

    AUIPC, // add upper intermideate to PC
    LUI,   // load upper intermideate

    BEQ,  // branch if equal
    BNE,  // branch if not equal
    BLT,  // branch if less than
    BGE,  // branch if greater or equal than
    BLTU, // branch if less than (unsigned)
    BGEU, // branch if greater or equal than (unsigned)

    JAL, // jump and link

    ECALL,  // make service request to the execution environment
    EBREAK, // return control to debugging environment

    FENCE, // order I/O and memory accesses

    URET, // return from user exception
    SRET, // return from supervisor exception
    MRET, // return from machine exception

    CSRRW,  // CSR read/write
    CSRRS,  // CSR read/set
    CSRRC,  // CSR read/clear
    CSRRWI, // CSR read/write immediate
    CSRRSI, // CSR read/set immediate
    CSRRCI, // CSR read/clear immediate
}
