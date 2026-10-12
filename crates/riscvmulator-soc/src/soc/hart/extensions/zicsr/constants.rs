use std::fmt::Display;

#[allow(non_camel_case_types)]
pub enum CSRRegisterName {
    // user lever (unprivilleged) CSRs

    // unprivilleged floating-point CSRs
    fflags = 0x001, // floating-point accrued exceptions
    frm = 0x002,    // floating-point dynamic rounding mode
    fcsr = 0x003,   // floating-point control and status register (frm + fflags)

    // unprivilleged vector CSRs
    vstart = 0x008, // vector start position
    vxsat = 0x009,  // fixed point accrued saturation flag
    vxrm = 0x00A,   // fixed point rounding mode
    vcsr = 0x00F,   // vector control and status register
    vl = 0xC20,     // vector length
    vtype = 0xC21,  // vector data type register
    vlenb = 0xC22,  // vector register length in bytes

    // unprivilleged Zicfiss extension CSR
    ssp = 0x011, // shadow stack pointer

    // unprivilleged entropy source extension CSR
    seed = 0x015, // seed for cryptographic random bit generators

    // unprivilleged Zcmt extension CSR
    jvt = 0x017, // table jump base vector and control register (CR)

    // unprivilleged counter/timers
    cycle = 0xC00,        // cycle counter for RDCYCLE instruction
    time = 0xC01,         // timer for RDTIME instruction
    instret = 0xC02, // instructions-retired counter for RDINSTRET instruction
    hpmcounter3 = 0xC03, // performance-monitoring counter
    hpmcounter4 = 0xC04, // performance-monitoring counter
    hpmcounter5 = 0xC05, // performance-monitoring counter
    hpmcounter6 = 0xC06, // performance-monitoring counter
    hpmcounter7 = 0xC07, // performance-monitoring counter
    hpmcounter8 = 0xC08, // performance-monitoring counter
    hpmcounter9 = 0xC09, // performance-monitoring counter
    hpmcounter10 = 0xC0A, // performance-monitoring counter
    hpmcounter11 = 0xC0B, // performance-monitoring counter
    hpmcounter12 = 0xC0C, // performance-monitoring counter
    hpmcounter13 = 0xC0D, // performance-monitoring counter
    hpmcounter14 = 0xC0E, // performance-monitoring counter
    hpmcounter15 = 0xC0F, // performance-monitoring counter
    hpmcounter16 = 0xC10, // performance-monitoring counter
    hpmcounter17 = 0xC11, // performance-monitoring counter
    hpmcounter18 = 0xC12, // performance-monitoring counter
    hpmcounter19 = 0xC13, // performance-monitoring counter
    hpmcounter20 = 0xC14, // performance-monitoring counter
    hpmcounter21 = 0xC15, // performance-monitoring counter
    hpmcounter22 = 0xC16, // performance-monitoring counter
    hpmcounter23 = 0xC17, // performance-monitoring counter
    hpmcounter24 = 0xC18, // performance-monitoring counter
    hpmcounter25 = 0xC19, // performance-monitoring counter
    hpmcounter26 = 0xC1A, // performance-monitoring counter
    hpmcounter27 = 0xC1B, // performance-monitoring counter
    hpmcounter28 = 0xC1C, // performance-monitoring counter
    hpmcounter29 = 0xC1D, // performance-monitoring counter
    hpmcounter30 = 0xC1E, // performance-monitoring counter
    hpmcounter31 = 0xC1F, // performance-monitoring counter

    cycleh = 0xC80,        // upper 32 bits of `cycle`, RV32 only
    timeh = 0xC81,         // upper 32 bits of `time`, RV32 only
    instreth = 0xC82,      // upper 32 bits of `instret`, RV32 only
    hpmcounter3h = 0xC83,  // upper 32 bits of `hpmcounter3`, RV32 only
    hpmcounter4h = 0xC84,  // upper 32 bits of `hpmcounter4`, RV32 only
    hpmcounter5h = 0xC85,  // upper 32 bits of `hpmcounter5`, RV32 only
    hpmcounter6h = 0xC86,  // upper 32 bits of `hpmcounter6`, RV32 only
    hpmcounter7h = 0xC87,  // upper 32 bits of `hpmcounter7`, RV32 only
    hpmcounter8h = 0xC88,  // upper 32 bits of `hpmcounter8`, RV32 only
    hpmcounter9h = 0xC89,  // upper 32 bits of `hpmcounter9`, RV32 only
    hpmcounter10h = 0xC8A, // upper 32 bits of `hpmcounter10`, RV32 only
    hpmcounter11h = 0xC8B, // upper 32 bits of `hpmcounter11`, RV32 only
    hpmcounter12h = 0xC8C, // upper 32 bits of `hpmcounter12`, RV32 only
    hpmcounter13h = 0xC8D, // upper 32 bits of `hpmcounter13`, RV32 only
    hpmcounter14h = 0xC8E, // upper 32 bits of `hpmcounter14`, RV32 only
    hpmcounter15h = 0xC8F, // upper 32 bits of `hpmcounter15`, RV32 only
    hpmcounter16h = 0xC90, // upper 32 bits of `hpmcounter16`, RV32 only
    hpmcounter17h = 0xC91, // upper 32 bits of `hpmcounter17`, RV32 only
    hpmcounter18h = 0xC92, // upper 32 bits of `hpmcounter18`, RV32 only
    hpmcounter19h = 0xC93, // upper 32 bits of `hpmcounter19`, RV32 only
    hpmcounter20h = 0xC94, // upper 32 bits of `hpmcounter20`, RV32 only
    hpmcounter21h = 0xC95, // upper 32 bits of `hpmcounter21`, RV32 only
    hpmcounter22h = 0xC96, // upper 32 bits of `hpmcounter22`, RV32 only
    hpmcounter23h = 0xC97, // upper 32 bits of `hpmcounter23`, RV32 only
    hpmcounter24h = 0xC98, // upper 32 bits of `hpmcounter24`, RV32 only
    hpmcounter25h = 0xC99, // upper 32 bits of `hpmcounter25`, RV32 only
    hpmcounter26h = 0xC9A, // upper 32 bits of `hpmcounter26`, RV32 only
    hpmcounter27h = 0xC9B, // upper 32 bits of `hpmcounter27`, RV32 only
    hpmcounter28h = 0xC9C, // upper 32 bits of `hpmcounter28`, RV32 only
    hpmcounter29h = 0xC9D, // upper 32 bits of `hpmcounter29`, RV32 only
    hpmcounter30h = 0xC9E, // upper 32 bits of `hpmcounter30`, RV32 only
    hpmcounter31h = 0xC9F, // upper 32 bits of `hpmcounter31`, RV32 only

    // supervisor-level CSRs

    // supervisor trap setup
    sstatus = 0x100,    // supervisor status register
    sie = 0x104,        // supervisor interrupt-enable register
    stvec = 0x105,      // supervisor trap handler base address
    scounteren = 0x106, // supervisor conter enable

    // supervisor configuration
    senvcfg = 0x10A, // supervisor environment configuration register

    // supervisor counter setup
    scountinhibit = 0x120, // supervisor counter-inhibit register

    // supervisor trap handling
    sscratch = 0x140,  // supervisor scratch register
    sepc = 0x141,      // supervisor exception program counter
    scause = 0x142,    // supervisor trap cause
    stval = 0x143,     // supervisor trap value
    sip = 0x144,       // supervisor interrupt pending
    scountovf = 0xDA0, // supervisor count overflow

    // supervisor indirect
    siselect = 0x150, // supervisor indirect register select
    sireg = 0x151,    // supervisor indirect register alias
    sireg2 = 0x152,   // supervisor indirect register alias 2
    sireg3 = 0x153,   // supervisor indirect register alias 3
    sireg4 = 0x155,   // supervisor indirect register alias 4
    sireg5 = 0x156,   // supervisor indirect register alias 5
    sireg6 = 0x157,   // supervisor indirect register alias 6

    // supervisor protection and translation
    satp = 0x180, // supervisor address translation and protection

    // supervisor timer compare
    stimecpm = 0x14D,  // supervisor timer compare
    stimecpmh = 0x15D, // upper 32 bits of `stimecpm`, RV32 only

    // supervisor debug/trace registers
    scontext = 0x5A8, // supervisor-mode context register

    // supervisor resource management configuration
    srmcfg = 0x181, // supervisor resource management configuration

    // supervisor state enable registers
    sstateen0 = 0x10C, // supervisor state enable 0 register
    sstateen1 = 0x10D, // supervisor state enable 1 register
    sstateen2 = 0x10E, // supervisor state enable 2 register
    sstateen3 = 0x10F, // supervisor state enable 3 register

    // supervisor control transfer records configuration
    sctrctl = 0x14E, // supervisor control transfer records control register
    sctrstatus = 0x14F, // supervisor control transfer records status register
    sctrdepth = 0x15F, // supervisor control transfer records depth register

    // hypervisor-level and virtual supervisor (VS) CSRs

    // hypervisor trap setup
    hstatus = 0x600,    // hypervisor status register
    hedeleg = 0x602,    // hypervisor exception delegation register
    hideleg = 0x603,    // hypervisor interrupt delegation register
    hie = 0x604,        // hypervisor interrupt-enable register
    hcounteren = 0x606, // hypervisor counter enable
    hgeie = 0x607,      // hypervisor guest external interrupt-enable register
    hedelegh = 0x612,   // 32 upper bits of `hedeleg`, RV32 only

    // hypervisor trap handling
    htval = 0x643,  // hypervisor trap value
    hip = 0x644,    // hypervisor interrupt pending
    hvip = 0x645,   // hypervisor interrupt pending
    htinst = 0x64A, // hypervisor count overflow
    hgeip = 0xE12,  // hypervisor interrupt pending

    // hypervisor configuration
    henvcfg = 0x60A, // hypervisor environment configuration register
    henvcfgh = 0x61A, // 32 upper bits of `henvcfg`, RV32 only

    // hypervisor protection and translation
    hgatp = 0x680, // hypervisor guest address translation and protection

    // hypervisor debug/trace registers
    hcontext = 0x6A8, // hypervisor-mode context register

    // hypervisor counter/timer virtualization registers
    htimedelta = 0x605,  // delta for VS/VU-mode timer
    htimedeltah = 0x615, // 32 upper bits of `htimedelta`, RV32 only

    // hypervisor state enable registers
    hstateen0 = 0x60C,  // hypervisor state enable 0 register
    hstateen1 = 0x60D,  // hypervisor state enable 1 register
    hstateen2 = 0x60E,  // hypervisor state enable 2 register
    hstateen3 = 0x60F,  // hypervisor state enable 3 register
    hstateen0h = 0x61C, // 32 upper bits of `hstateen0`, RV32 only
    hstateen1h = 0x61D, // 32 upper bits of `hstateen1`, RV32 only
    hstateen2h = 0x61E, // 32 upper bits of `hstateen2`, RV32 only
    hstateen3h = 0x61F, // 32 upper bits of `hstateen3`, RV32 only

    // virtual supervisor (VS) registers
    // virtual supervisor (VS) registers
    vstatus = 0x200,   // virtual supervisor status register
    vsie = 0x204,      // virtual supervisor interrupt-enable register
    vstvec = 0x205,    // virtual supervisor trap handler base address
    vsscratch = 0x240, // virtual supervisor scratch register
    vsepc = 0x241,     // virtual supervisor exception program counter
    vscause = 0x242,   // virtual supervisor trap cause
    vstval = 0x243,    // virtual supervisor trap value
    vsip = 0x244,      // virtual supervisor interrupt pending
    vsatp = 0x280,     // virtual supervisor address translation and protection

    // virtual supervisor indirect
    vsiselect = 0x250, // virtual supervisor indirect register select
    vsireg = 0x251,    // virtual supervisor indirect register alias
    vsireg2 = 0x252,   // virtual supervisor indirect register alias 2
    vsireg3 = 0x253,   // virtual supervisor indirect register alias 3
    vsireg4 = 0x255,   // virtual supervisor indirect register alias 4
    vsireg5 = 0x256,   // virtual supervisor indirect register alias 5
    vsireg6 = 0x257,   // virtual supervisor indirect register alias 6

    // virtual supervisor timer compare
    vstimecpm = 0x24D,  // virtual supervisor timer compare
    vstimecpmh = 0x25D, // virtual upper 32 bits of `vstimecpm`, RV32 only

    // virtual supervisor control transfer records configuration
    vsctrctl = 0x24E, // virtual supervisor control transfer records control register

    // machine-level CSRs

    // machine information registers
    mvendorid = 0xF11,  // vendor ID
    marchid = 0xF12,    // architecture ID
    mimpid = 0xF13,     // implementation ID
    mhartid = 0xF14,    // hardware thread ID
    mconfigptr = 0xF15, // pointer to configuration data structure

    // machine trap setup
    mstatus = 0x300,    // machine status register
    misa = 0x301,       // ISA and extensions
    medeleg = 0x302,    // machine exception delegation register
    mideleg = 0x303,    // machine interrupt delegation register
    mie = 0x304,        // machine interrupt-enable register
    mtvec = 0x305,      // machine trap-handler base address
    mcounteren = 0x306, // machine counter enable
    mstatush = 0x310,   // 32 upper bits of `mstatus`, RV32 only
    medelegh = 0x312,   // 32 upper bits of `medeleg`, RV32 only

    // machine trap handling
    mscratch = 0x340, // machine scratch register
    mepc = 0x341,     // machine exception program counter
    mcause = 0x342,   // machine trap cause
    mtval = 0x343,    // machine trap value
    mip = 0x344,      // machine interrupt pending
    mtinst = 0x34A,   // machine
    mtval2 = 0x34B,   // machine second trap value

    // machine indirect
    miselect = 0x350, // machine indirect register select
    mireg = 0x351,    // machine indirect register alias
    mireg2 = 0x352,   // machine indirect register alias 2
    mireg3 = 0x353,   // machine indirect register alias 3
    mireg4 = 0x355,   // machine indirect register alias 4
    mireg5 = 0x356,   // machine indirect register alias 5
    mireg6 = 0x357,   // machine indirect register alias 6

    // machine configuration
    menvcfg = 0x30A, // machine environment configuration register
    menvcfgh = 0x31A, // 32 upper bits of `menvcfg`, RV32 only
    mseccfg = 0x747, // machine security configuration register
    mseccfgh = 0x757, // 32 upper bits of `mseccfg`, RV32 only

    // machine memory protection
    pmpcfg0 = 0x3A0,  // physical memory protection configuraiton
    pmpcfg1 = 0x3A1,  // physical memory protection configuraiton, RV32 only
    pmpcfg2 = 0x3A2,  // physical memory protection configuraiton
    pmpcfg3 = 0x3A3,  // physical memory protection configuraiton, RV32 only
    pmpcfg4 = 0x3A4,  // physical memory protection configuraiton
    pmpcfg5 = 0x3A5,  // physical memory protection configuraiton, RV32 only
    pmpcfg6 = 0x3A6,  // physical memory protection configuraiton
    pmpcfg7 = 0x3A7,  // physical memory protection configuraiton, RV32 only
    pmpcfg8 = 0x3A8,  // physical memory protection configuraiton
    pmpcfg9 = 0x3A9,  // physical memory protection configuraiton, RV32 only
    pmpcfg10 = 0x3AA, // physical memory protection configuraiton
    pmpcfg11 = 0x3AB, // physical memory protection configuraiton, RV32 only
    pmpcfg12 = 0x3AC, // physical memory protection configuraiton
    pmpcfg13 = 0x3AD, // physical memory protection configuraiton, RV32 only
    pmpcfg14 = 0x3AE, // physical memory protection configuraiton
    pmpcfg15 = 0x3AF, // physical memory protection configuraiton, RV32 only
    pmaddr0 = 0x3B0,  // physical memory protection address register
    pmaddr1 = 0x3B1,  // physical memory protection address register
    pmaddr2 = 0x3B2,  // physical memory protection address register
    pmaddr3 = 0x3B3,  // physical memory protection address register
    pmaddr4 = 0x3B4,  // physical memory protection address register
    pmaddr5 = 0x3B5,  // physical memory protection address register
    pmaddr6 = 0x3B6,  // physical memory protection address register
    pmaddr7 = 0x3B7,  // physical memory protection address register
    pmaddr8 = 0x3B8,  // physical memory protection address register
    pmaddr9 = 0x3B9,  // physical memory protection address register
    pmaddr10 = 0x3BA, // physical memory protection address register
    pmaddr11 = 0x3BB, // physical memory protection address register
    pmaddr12 = 0x3BC, // physical memory protection address register
    pmaddr13 = 0x3BD, // physical memory protection address register
    pmaddr14 = 0x3BE, // physical memory protection address register
    pmaddr15 = 0x3BF, // physical memory protection address register
    pmaddr16 = 0x3C0, // physical memory protection address register
    pmaddr17 = 0x3C1, // physical memory protection address register
    pmaddr18 = 0x3C2, // physical memory protection address register
    pmaddr19 = 0x3C3, // physical memory protection address register
    pmaddr20 = 0x3C4, // physical memory protection address register
    pmaddr21 = 0x3C5, // physical memory protection address register
    pmaddr22 = 0x3C6, // physical memory protection address register
    pmaddr23 = 0x3C7, // physical memory protection address register
    pmaddr24 = 0x3C8, // physical memory protection address register
    pmaddr25 = 0x3C9, // physical memory protection address register
    pmaddr26 = 0x3CA, // physical memory protection address register
    pmaddr27 = 0x3CB, // physical memory protection address register
    pmaddr28 = 0x3CC, // physical memory protection address register
    pmaddr29 = 0x3CD, // physical memory protection address register
    pmaddr30 = 0x3CE, // physical memory protection address register
    pmaddr31 = 0x3CF, // physical memory protection address register
    pmaddr32 = 0x3D0, // physical memory protection address register
    pmaddr33 = 0x3D1, // physical memory protection address register
    pmaddr34 = 0x3D2, // physical memory protection address register
    pmaddr35 = 0x3D3, // physical memory protection address register
    pmaddr36 = 0x3D4, // physical memory protection address register
    pmaddr37 = 0x3D5, // physical memory protection address register
    pmaddr38 = 0x3D6, // physical memory protection address register
    pmaddr39 = 0x3D7, // physical memory protection address register
    pmaddr40 = 0x3D8, // physical memory protection address register
    pmaddr41 = 0x3D9, // physical memory protection address register
    pmaddr42 = 0x3DA, // physical memory protection address register
    pmaddr43 = 0x3DB, // physical memory protection address register
    pmaddr44 = 0x3DC, // physical memory protection address register
    pmaddr45 = 0x3DD, // physical memory protection address register
    pmaddr46 = 0x3DE, // physical memory protection address register
    pmaddr47 = 0x3DF, // physical memory protection address register
    pmaddr48 = 0x3E0, // physical memory protection address register
    pmaddr49 = 0x3E1, // physical memory protection address register
    pmaddr50 = 0x3E2, // physical memory protection address register
    pmaddr51 = 0x3E3, // physical memory protection address register
    pmaddr52 = 0x3E4, // physical memory protection address register
    pmaddr53 = 0x3E5, // physical memory protection address register
    pmaddr54 = 0x3E6, // physical memory protection address register
    pmaddr55 = 0x3E7, // physical memory protection address register
    pmaddr56 = 0x3E8, // physical memory protection address register
    pmaddr57 = 0x3E9, // physical memory protection address register
    pmaddr58 = 0x3EA, // physical memory protection address register
    pmaddr59 = 0x3EB, // physical memory protection address register
    pmaddr60 = 0x3EC, // physical memory protection address register
    pmaddr61 = 0x3ED, // physical memory protection address register
    pmaddr62 = 0x3EE, // physical memory protection address register
    pmaddr63 = 0x3EF, // physical memory protection address register

    // machine state enable registers
    mstateen0 = 0x30C,  // machine state enable 0 register
    mstateen1 = 0x30D,  // machine state enable 1 register
    mstateen2 = 0x30E,  // machine state enable 2 register
    mstateen3 = 0x30F,  // machine state enable 3 register
    mstateen0h = 0x31C, // 32 upper bits of `mstateen0`, RV32 only
    mstateen1h = 0x31D, // 32 upper bits of `mstateen1`, RV32 only
    mstateen2h = 0x31E, // 32 upper bits of `mstateen2`, RV32 only
    mstateen3h = 0x31F, // 32 upper bits of `mstateen3`, RV32 only

    // machine non-maskable interrupt handling
    mnscratch = 0x740, // resumable NMI scratch register
    mnepc = 0x741,     // resumable NMI program counter
    mncause = 0x742,   // resumable NMI cause
    mnstatus = 0x744,  // resumable NMI status

    // machine counter/timers
    mcycle = 0xB00,        // machine cycle counter
    minstret = 0xB02,      // machine instructions-retired counter
    mhpmcounter3 = 0xB03,  // machine performance-monitoring counter
    mhpmcounter4 = 0xB04,  // machine performance-monitoring counter
    mhpmcounter5 = 0xB05,  // machine performance-monitoring counter
    mhpmcounter6 = 0xB06,  // machine performance-monitoring counter
    mhpmcounter7 = 0xB07,  // machine performance-monitoring counter
    mhpmcounter8 = 0xB08,  // machine performance-monitoring counter
    mhpmcounter9 = 0xB09,  // machine performance-monitoring counter
    mhpmcounter10 = 0xB0A, // machine performance-monitoring counter
    mhpmcounter11 = 0xB0B, // machine performance-monitoring counter
    mhpmcounter12 = 0xB0C, // machine performance-monitoring counter
    mhpmcounter13 = 0xB0D, // machine performance-monitoring counter
    mhpmcounter14 = 0xB0E, // machine performance-monitoring counter
    mhpmcounter15 = 0xB0F, // machine performance-monitoring counter
    mhpmcounter16 = 0xB10, // machine performance-monitoring counter
    mhpmcounter17 = 0xB11, // machine performance-monitoring counter
    mhpmcounter18 = 0xB12, // machine performance-monitoring counter
    mhpmcounter19 = 0xB13, // machine performance-monitoring counter
    mhpmcounter20 = 0xB14, // machine performance-monitoring counter
    mhpmcounter21 = 0xB15, // machine performance-monitoring counter
    mhpmcounter22 = 0xB16, // machine performance-monitoring counter
    mhpmcounter23 = 0xB17, // machine performance-monitoring counter
    mhpmcounter24 = 0xB18, // machine performance-monitoring counter
    mhpmcounter25 = 0xB19, // machine performance-monitoring counter
    mhpmcounter26 = 0xB1A, // machine performance-monitoring counter
    mhpmcounter27 = 0xB1B, // machine performance-monitoring counter
    mhpmcounter28 = 0xB1C, // machine performance-monitoring counter
    mhpmcounter29 = 0xB1D, // machine performance-monitoring counter
    mhpmcounter30 = 0xB1E, // machine performance-monitoring counter
    mhpmcounter31 = 0xB1F, // machine performance-monitoring counter

    mcycleh = 0xB80,       // upper 32 bits of `mcycle`, RV32 only
    minstreth = 0xB82,     // upper 32 bits of `minstret`, RV32 only
    mhpmcounter3h = 0xB83, // upper 32 bits of `mhpmcounter3`, RV32 only
    mhpmcounter4h = 0xB84, // upper 32 bits of `mhpmcounter4`, RV32 only
    mhpmcounter5h = 0xB85, // upper 32 bits of `mhpmcounter5`, RV32 only
    mhpmcounter6h = 0xB86, // upper 32 bits of `mhpmcounter6`, RV32 only
    mhpmcounter7h = 0xB87, // upper 32 bits of `mhpmcounter7`, RV32 only
    mhpmcounter8h = 0xB88, // upper 32 bits of `mhpmcounter8`, RV32 only
    mhpmcounter9h = 0xB89, // upper 32 bits of `mhpmcounter9`, RV32 only
    mhpmcounter10h = 0xB8A, // upper 32 bits of `mhpmcounter10`, RV32 only
    mhpmcounter11h = 0xB8B, // upper 32 bits of `mhpmcounter11`, RV32 only
    mhpmcounter12h = 0xB8C, // upper 32 bits of `mhpmcounter12`, RV32 only
    mhpmcounter13h = 0xB8D, // upper 32 bits of `mhpmcounter13`, RV32 only
    mhpmcounter14h = 0xB8E, // upper 32 bits of `mhpmcounter14`, RV32 only
    mhpmcounter15h = 0xB8F, // upper 32 bits of `mhpmcounter15`, RV32 only
    mhpmcounter16h = 0xB90, // upper 32 bits of `mhpmcounter16`, RV32 only
    mhpmcounter17h = 0xB91, // upper 32 bits of `mhpmcounter17`, RV32 only
    mhpmcounter18h = 0xB92, // upper 32 bits of `mhpmcounter18`, RV32 only
    mhpmcounter19h = 0xB93, // upper 32 bits of `mhpmcounter19`, RV32 only
    mhpmcounter20h = 0xB94, // upper 32 bits of `mhpmcounter20`, RV32 only
    mhpmcounter21h = 0xB95, // upper 32 bits of `mhpmcounter21`, RV32 only
    mhpmcounter22h = 0xB96, // upper 32 bits of `mhpmcounter22`, RV32 only
    mhpmcounter23h = 0xB97, // upper 32 bits of `mhpmcounter23`, RV32 only
    mhpmcounter24h = 0xB98, // upper 32 bits of `mhpmcounter24`, RV32 only
    mhpmcounter25h = 0xB99, // upper 32 bits of `mhpmcounter25`, RV32 only
    mhpmcounter26h = 0xB9A, // upper 32 bits of `mhpmcounter26`, RV32 only
    mhpmcounter27h = 0xB9B, // upper 32 bits of `mhpmcounter27`, RV32 only
    mhpmcounter28h = 0xB9C, // upper 32 bits of `mhpmcounter28`, RV32 only
    mhpmcounter29h = 0xB9D, // upper 32 bits of `mhpmcounter29`, RV32 only
    mhpmcounter30h = 0xB9E, // upper 32 bits of `mhpmcounter30`, RV32 only
    mhpmcounter31h = 0xB9F, // upper 32 bits of `mhpmcounter31`, RV32 only

    // machine counter setup
    mcountinhibit = 0x320, // machine counter-inhibit register
    mcyclecfg = 0x321,     // machine cycle-counter configuration register
    minstretcfg = 0x322,   // machine insert counter configuration register
    mhpmevent3 = 0x323,    // machine performance-monitoring event selector
    mhpmevent4 = 0x324,    // machine performance-monitoring event selector
    mhpmevent5 = 0x325,    // machine performance-monitoring event selector
    mhpmevent6 = 0x326,    // machine performance-monitoring event selector
    mhpmevent7 = 0x327,    // machine performance-monitoring event selector
    mhpmevent8 = 0x328,    // machine performance-monitoring event selector
    mhpmevent9 = 0x329,    // machine performance-monitoring event selector
    mhpmevent10 = 0x32A,   // machine performance-monitoring event selector
    mhpmevent11 = 0x32B,   // machine performance-monitoring event selector
    mhpmevent12 = 0x32C,   // machine performance-monitoring event selector
    mhpmevent13 = 0x32D,   // machine performance-monitoring event selector
    mhpmevent14 = 0x32E,   // machine performance-monitoring event selector
    mhpmevent15 = 0x32F,   // machine performance-monitoring event selector
    mhpmevent16 = 0x330,   // machine performance-monitoring event selector
    mhpmevent17 = 0x331,   // machine performance-monitoring event selector
    mhpmevent18 = 0x332,   // machine performance-monitoring event selector
    mhpmevent19 = 0x333,   // machine performance-monitoring event selector
    mhpmevent20 = 0x334,   // machine performance-monitoring event selector
    mhpmevent21 = 0x335,   // machine performance-monitoring event selector
    mhpmevent22 = 0x336,   // machine performance-monitoring event selector
    mhpmevent23 = 0x337,   // machine performance-monitoring event selector
    mhpmevent24 = 0x338,   // machine performance-monitoring event selector
    mhpmevent25 = 0x339,   // machine performance-monitoring event selector
    mhpmevent26 = 0x33A,   // machine performance-monitoring event selector
    mhpmevent27 = 0x33B,   // machine performance-monitoring event selector
    mhpmevent28 = 0x33C,   // machine performance-monitoring event selector
    mhpmevent29 = 0x33D,   // machine performance-monitoring event selector
    mhpmevent30 = 0x33E,   // machine performance-monitoring event selector
    mhpmevent31 = 0x33F,   // machine performance-monitoring event selector

    mcyclecfgh = 0x721, // upper 32 bits of `mcyclecfg`, RV32 only
    minstretcfgh = 0x722, // upper 32 bits of `minstretcfg`, RV32 only
    mhpmevent3h = 0x723, // upper 32 bits of `mhpmevent3`, RV32 only
    mhpmevent4h = 0x724, // upper 32 bits of `mhpmevent4`, RV32 only
    mhpmevent5h = 0x725, // upper 32 bits of `mhpmevent5`, RV32 only
    mhpmevent6h = 0x726, // upper 32 bits of `mhpmevent6`, RV32 only
    mhpmevent7h = 0x727, // upper 32 bits of `mhpmevent7`, RV32 only
    mhpmevent8h = 0x728, // upper 32 bits of `mhpmevent8`, RV32 only
    mhpmevent9h = 0x729, // upper 32 bits of `mhpmevent9`, RV32 only
    mhpmevent10h = 0x72A, // upper 32 bits of `mhpmevent10`, RV32 only
    mhpmevent11h = 0x72B, // upper 32 bits of `mhpmevent11`, RV32 only
    mhpmevent12h = 0x72C, // upper 32 bits of `mhpmevent12`, RV32 only
    mhpmevent13h = 0x72D, // upper 32 bits of `mhpmevent13`, RV32 only
    mhpmevent14h = 0x72E, // upper 32 bits of `mhpmevent14`, RV32 only
    mhpmevent15h = 0x72F, // upper 32 bits of `mhpmevent15`, RV32 only
    mhpmevent16h = 0x730, // upper 32 bits of `mhpmevent16`, RV32 only
    mhpmevent17h = 0x731, // upper 32 bits of `mhpmevent17`, RV32 only
    mhpmevent18h = 0x732, // upper 32 bits of `mhpmevent18`, RV32 only
    mhpmevent19h = 0x733, // upper 32 bits of `mhpmevent19`, RV32 only
    mhpmevent20h = 0x734, // upper 32 bits of `mhpmevent20`, RV32 only
    mhpmevent21h = 0x735, // upper 32 bits of `mhpmevent21`, RV32 only
    mhpmevent22h = 0x736, // upper 32 bits of `mhpmevent22`, RV32 only
    mhpmevent23h = 0x737, // upper 32 bits of `mhpmevent23`, RV32 only
    mhpmevent24h = 0x738, // upper 32 bits of `mhpmevent24`, RV32 only
    mhpmevent25h = 0x739, // upper 32 bits of `mhpmevent25`, RV32 only
    mhpmevent26h = 0x73A, // upper 32 bits of `mhpmevent26`, RV32 only
    mhpmevent27h = 0x73B, // upper 32 bits of `mhpmevent27`, RV32 only
    mhpmevent28h = 0x73C, // upper 32 bits of `mhpmevent28`, RV32 only
    mhpmevent29h = 0x73D, // upper 32 bits of `mhpmevent29`, RV32 only
    mhpmevent30h = 0x73E, // upper 32 bits of `mhpmevent30`, RV32 only
    mhpmevent31h = 0x73F, // upper 32 bits of `mhpmevent31`, RV32 only

    // machine control transfer records configuration
    mctrctl = 0x34E, // machine control transfer records control register

    // machine/debug level CSRs

    // debug/trace registers (shared with debug mode)
    tselect = 0x7A0,  // debug/trace trigger register select
    tdata1 = 0x7A1,   // first debug/trace trigger data register
    tdata2 = 0x7A2,   // second debug/trace trigger data register
    tdata3 = 0x7A3,   // third debug/trace trigger data register
    mcontext = 0x7A8, // machine-mode context register

    // debug mode registers

    // debug mode registers
    dcsr = 0x7B0,      // debug control and status register
    dpc = 0x7B1,       // debug program counter
    dscratch0 = 0x7B2, // debug scratch register 0
    dscratch1 = 0x7B3, // debug scratch register 1
}

impl TryFrom<usize> for CSRRegisterName {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            // user lever (unprivilleged) CSRs

            // unprivilleged floating-point CSRs
            0x001 => Ok(CSRRegisterName::fflags), // floating-point accrued exceptions
            0x002 => Ok(CSRRegisterName::frm), // floating-point dynamic rounding mode
            0x003 => Ok(CSRRegisterName::fcsr), // floating-point control and status register (frm + fflags)

            // unprivilleged vector CSRs
            0x008 => Ok(CSRRegisterName::vstart), // vector start position
            0x009 => Ok(CSRRegisterName::vxsat), // fixed point accrued saturation flag
            0x00A => Ok(CSRRegisterName::vxrm),  // fixed point rounding mode
            0x00F => Ok(CSRRegisterName::vcsr), // vector control and status register
            0xC20 => Ok(CSRRegisterName::vl),   // vector length
            0xC21 => Ok(CSRRegisterName::vtype), // vector data type register
            0xC22 => Ok(CSRRegisterName::vlenb), // vector register length in bytes

            // unprivilleged Zicfiss extension CSR
            0x011 => Ok(CSRRegisterName::ssp), // shadow stack pointer

            // unprivilleged entropy source extension CSR
            0x015 => Ok(CSRRegisterName::seed), // seed for cryptographic random bit generators

            // unprivilleged Zcmt extension CSR
            0x017 => Ok(CSRRegisterName::jvt), // table jump base vector and control register (CR)

            // unprivilleged counter/timers
            0xC00 => Ok(CSRRegisterName::cycle), // cycle counter for RDCYCLE instruction
            0xC01 => Ok(CSRRegisterName::time),  // timer for RDTIME instruction
            0xC02 => Ok(CSRRegisterName::instret), // instructions-retired counter for RDINSTRET instruction
            0xC03 => Ok(CSRRegisterName::hpmcounter3), // performance-monitoring counter
            0xC04 => Ok(CSRRegisterName::hpmcounter4), // performance-monitoring counter
            0xC05 => Ok(CSRRegisterName::hpmcounter5), // performance-monitoring counter
            0xC06 => Ok(CSRRegisterName::hpmcounter6), // performance-monitoring counter
            0xC07 => Ok(CSRRegisterName::hpmcounter7), // performance-monitoring counter
            0xC08 => Ok(CSRRegisterName::hpmcounter8), // performance-monitoring counter
            0xC09 => Ok(CSRRegisterName::hpmcounter9), // performance-monitoring counter
            0xC0A => Ok(CSRRegisterName::hpmcounter10), // performance-monitoring counter
            0xC0B => Ok(CSRRegisterName::hpmcounter11), // performance-monitoring counter
            0xC0C => Ok(CSRRegisterName::hpmcounter12), // performance-monitoring counter
            0xC0D => Ok(CSRRegisterName::hpmcounter13), // performance-monitoring counter
            0xC0E => Ok(CSRRegisterName::hpmcounter14), // performance-monitoring counter
            0xC0F => Ok(CSRRegisterName::hpmcounter15), // performance-monitoring counter
            0xC10 => Ok(CSRRegisterName::hpmcounter16), // performance-monitoring counter
            0xC11 => Ok(CSRRegisterName::hpmcounter17), // performance-monitoring counter
            0xC12 => Ok(CSRRegisterName::hpmcounter18), // performance-monitoring counter
            0xC13 => Ok(CSRRegisterName::hpmcounter19), // performance-monitoring counter
            0xC14 => Ok(CSRRegisterName::hpmcounter20), // performance-monitoring counter
            0xC15 => Ok(CSRRegisterName::hpmcounter21), // performance-monitoring counter
            0xC16 => Ok(CSRRegisterName::hpmcounter22), // performance-monitoring counter
            0xC17 => Ok(CSRRegisterName::hpmcounter23), // performance-monitoring counter
            0xC18 => Ok(CSRRegisterName::hpmcounter24), // performance-monitoring counter
            0xC19 => Ok(CSRRegisterName::hpmcounter25), // performance-monitoring counter
            0xC1A => Ok(CSRRegisterName::hpmcounter26), // performance-monitoring counter
            0xC1B => Ok(CSRRegisterName::hpmcounter27), // performance-monitoring counter
            0xC1C => Ok(CSRRegisterName::hpmcounter28), // performance-monitoring counter
            0xC1D => Ok(CSRRegisterName::hpmcounter29), // performance-monitoring counter
            0xC1E => Ok(CSRRegisterName::hpmcounter30), // performance-monitoring counter
            0xC1F => Ok(CSRRegisterName::hpmcounter31), // performance-monitoring counter

            0xC80 => Ok(CSRRegisterName::cycleh), // upper 32 bits of `cycle`, RV32 only
            0xC81 => Ok(CSRRegisterName::timeh), // upper 32 bits of `time`, RV32 only
            0xC82 => Ok(CSRRegisterName::instreth), // upper 32 bits of `instret`, RV32 only
            0xC83 => Ok(CSRRegisterName::hpmcounter3h), // upper 32 bits of `hpmcounter3`, RV32 only
            0xC84 => Ok(CSRRegisterName::hpmcounter4h), // upper 32 bits of `hpmcounter4`, RV32 only
            0xC85 => Ok(CSRRegisterName::hpmcounter5h), // upper 32 bits of `hpmcounter5`, RV32 only
            0xC86 => Ok(CSRRegisterName::hpmcounter6h), // upper 32 bits of `hpmcounter6`, RV32 only
            0xC87 => Ok(CSRRegisterName::hpmcounter7h), // upper 32 bits of `hpmcounter7`, RV32 only
            0xC88 => Ok(CSRRegisterName::hpmcounter8h), // upper 32 bits of `hpmcounter8`, RV32 only
            0xC89 => Ok(CSRRegisterName::hpmcounter9h), // upper 32 bits of `hpmcounter9`, RV32 only
            0xC8A => Ok(CSRRegisterName::hpmcounter10h), // upper 32 bits of `hpmcounter10`, RV32 only
            0xC8B => Ok(CSRRegisterName::hpmcounter11h), // upper 32 bits of `hpmcounter11`, RV32 only
            0xC8C => Ok(CSRRegisterName::hpmcounter12h), // upper 32 bits of `hpmcounter12`, RV32 only
            0xC8D => Ok(CSRRegisterName::hpmcounter13h), // upper 32 bits of `hpmcounter13`, RV32 only
            0xC8E => Ok(CSRRegisterName::hpmcounter14h), // upper 32 bits of `hpmcounter14`, RV32 only
            0xC8F => Ok(CSRRegisterName::hpmcounter15h), // upper 32 bits of `hpmcounter15`, RV32 only
            0xC90 => Ok(CSRRegisterName::hpmcounter16h), // upper 32 bits of `hpmcounter16`, RV32 only
            0xC91 => Ok(CSRRegisterName::hpmcounter17h), // upper 32 bits of `hpmcounter17`, RV32 only
            0xC92 => Ok(CSRRegisterName::hpmcounter18h), // upper 32 bits of `hpmcounter18`, RV32 only
            0xC93 => Ok(CSRRegisterName::hpmcounter19h), // upper 32 bits of `hpmcounter19`, RV32 only
            0xC94 => Ok(CSRRegisterName::hpmcounter20h), // upper 32 bits of `hpmcounter20`, RV32 only
            0xC95 => Ok(CSRRegisterName::hpmcounter21h), // upper 32 bits of `hpmcounter21`, RV32 only
            0xC96 => Ok(CSRRegisterName::hpmcounter22h), // upper 32 bits of `hpmcounter22`, RV32 only
            0xC97 => Ok(CSRRegisterName::hpmcounter23h), // upper 32 bits of `hpmcounter23`, RV32 only
            0xC98 => Ok(CSRRegisterName::hpmcounter24h), // upper 32 bits of `hpmcounter24`, RV32 only
            0xC99 => Ok(CSRRegisterName::hpmcounter25h), // upper 32 bits of `hpmcounter25`, RV32 only
            0xC9A => Ok(CSRRegisterName::hpmcounter26h), // upper 32 bits of `hpmcounter26`, RV32 only
            0xC9B => Ok(CSRRegisterName::hpmcounter27h), // upper 32 bits of `hpmcounter27`, RV32 only
            0xC9C => Ok(CSRRegisterName::hpmcounter28h), // upper 32 bits of `hpmcounter28`, RV32 only
            0xC9D => Ok(CSRRegisterName::hpmcounter29h), // upper 32 bits of `hpmcounter29`, RV32 only
            0xC9E => Ok(CSRRegisterName::hpmcounter30h), // upper 32 bits of `hpmcounter30`, RV32 only
            0xC9F => Ok(CSRRegisterName::hpmcounter31h), // upper 32 bits of `hpmcounter31`, RV32 only

            // supervisor-level CSRs

            // supervisor trap setup
            0x100 => Ok(CSRRegisterName::sstatus), // supervisor status register
            0x104 => Ok(CSRRegisterName::sie), // supervisor interrupt-enable register
            0x105 => Ok(CSRRegisterName::stvec), // supervisor trap handler base address
            0x106 => Ok(CSRRegisterName::scounteren), // supervisor conter enable

            // supervisor configuration
            0x10A => Ok(CSRRegisterName::senvcfg), // supervisor environment configuration register

            // supervisor counter setup
            0x120 => Ok(CSRRegisterName::scountinhibit), // supervisor counter-inhibit register

            // supervisor trap handling
            0x140 => Ok(CSRRegisterName::sscratch), // supervisor scratch register
            0x141 => Ok(CSRRegisterName::sepc), // supervisor exception program counter
            0x142 => Ok(CSRRegisterName::scause), // supervisor trap cause
            0x143 => Ok(CSRRegisterName::stval), // supervisor trap value
            0x144 => Ok(CSRRegisterName::sip),  // supervisor interrupt pending
            0xDA0 => Ok(CSRRegisterName::scountovf), // supervisor count overflow

            // supervisor indirect
            0x150 => Ok(CSRRegisterName::siselect), // supervisor indirect register select
            0x151 => Ok(CSRRegisterName::sireg), // supervisor indirect register alias
            0x152 => Ok(CSRRegisterName::sireg2), // supervisor indirect register alias 2
            0x153 => Ok(CSRRegisterName::sireg3), // supervisor indirect register alias 3
            0x155 => Ok(CSRRegisterName::sireg4), // supervisor indirect register alias 4
            0x156 => Ok(CSRRegisterName::sireg5), // supervisor indirect register alias 5
            0x157 => Ok(CSRRegisterName::sireg6), // supervisor indirect register alias 6

            // supervisor protection and translation
            0x180 => Ok(CSRRegisterName::satp), // supervisor address translation and protection

            // supervisor timer compare
            0x14D => Ok(CSRRegisterName::stimecpm), // supervisor timer compare
            0x15D => Ok(CSRRegisterName::stimecpmh), // upper 32 bits of `stimecpm`, RV32 only

            // supervisor debug/trace registers
            0x5A8 => Ok(CSRRegisterName::scontext), // supervisor-mode context register

            // supervisor resource management configuration
            0x181 => Ok(CSRRegisterName::srmcfg), // supervisor resource management configuration

            // supervisor state enable registers
            0x10C => Ok(CSRRegisterName::sstateen0), // supervisor state enable 0 register
            0x10D => Ok(CSRRegisterName::sstateen1), // supervisor state enable 1 register
            0x10E => Ok(CSRRegisterName::sstateen2), // supervisor state enable 2 register
            0x10F => Ok(CSRRegisterName::sstateen3), // supervisor state enable 3 register

            // supervisor control transfer records configuration
            0x14E => Ok(CSRRegisterName::sctrctl), // supervisor control transfer records control register
            0x14F => Ok(CSRRegisterName::sctrstatus), // supervisor control transfer records status register
            0x15F => Ok(CSRRegisterName::sctrdepth), // supervisor control transfer records depth register

            // hypervisor-level and virtual supervisor (VS) CSRs

            // hypervisor trap setup
            0x600 => Ok(CSRRegisterName::hstatus), // hypervisor status register
            0x602 => Ok(CSRRegisterName::hedeleg), // hypervisor exception delegation register
            0x603 => Ok(CSRRegisterName::hideleg), // hypervisor interrupt delegation register
            0x604 => Ok(CSRRegisterName::hie), // hypervisor interrupt-enable register
            0x606 => Ok(CSRRegisterName::hcounteren), // hypervisor counter enable
            0x607 => Ok(CSRRegisterName::hgeie), // hypervisor guest external interrupt-enable register
            0x612 => Ok(CSRRegisterName::hedelegh), // 32 upper bits of `hedeleg`, RV32 only

            // hypervisor trap handling
            0x643 => Ok(CSRRegisterName::htval), // hypervisor trap value
            0x644 => Ok(CSRRegisterName::hip),   // hypervisor interrupt pending
            0x645 => Ok(CSRRegisterName::hvip),  // hypervisor interrupt pending
            0x64A => Ok(CSRRegisterName::htinst), // hypervisor count overflow
            0xE12 => Ok(CSRRegisterName::hgeip), // hypervisor interrupt pending

            // hypervisor configuration
            0x60A => Ok(CSRRegisterName::henvcfg), // hypervisor environment configuration register
            0x61A => Ok(CSRRegisterName::henvcfgh), // 32 upper bits of `henvcfg`, RV32 only

            // hypervisor protection and translation
            0x680 => Ok(CSRRegisterName::hgatp), // hypervisor guest address translation and protection

            // hypervisor debug/trace registers
            0x6A8 => Ok(CSRRegisterName::hcontext), // hypervisor-mode context register

            // hypervisor counter/timer virtualization registers
            0x605 => Ok(CSRRegisterName::htimedelta), // delta for VS/VU-mode timer
            0x615 => Ok(CSRRegisterName::htimedeltah), // 32 upper bits of `htimedelta`, RV32 only

            // hypervisor state enable registers
            0x60C => Ok(CSRRegisterName::hstateen0), // hypervisor state enable 0 register
            0x60D => Ok(CSRRegisterName::hstateen1), // hypervisor state enable 1 register
            0x60E => Ok(CSRRegisterName::hstateen2), // hypervisor state enable 2 register
            0x60F => Ok(CSRRegisterName::hstateen3), // hypervisor state enable 3 register
            0x61C => Ok(CSRRegisterName::hstateen0h), // 32 upper bits of `hstateen0`, RV32 only
            0x61D => Ok(CSRRegisterName::hstateen1h), // 32 upper bits of `hstateen1`, RV32 only
            0x61E => Ok(CSRRegisterName::hstateen2h), // 32 upper bits of `hstateen2`, RV32 only
            0x61F => Ok(CSRRegisterName::hstateen3h), // 32 upper bits of `hstateen3`, RV32 only

            // virtual supervisor (VS) registers
            // virtual supervisor (VS) registers
            0x200 => Ok(CSRRegisterName::vstatus), // virtual supervisor status register
            0x204 => Ok(CSRRegisterName::vsie), // virtual supervisor interrupt-enable register
            0x205 => Ok(CSRRegisterName::vstvec), // virtual supervisor trap handler base address
            0x240 => Ok(CSRRegisterName::vsscratch), // virtual supervisor scratch register
            0x241 => Ok(CSRRegisterName::vsepc), // virtual supervisor exception program counter
            0x242 => Ok(CSRRegisterName::vscause), // virtual supervisor trap cause
            0x243 => Ok(CSRRegisterName::vstval), // virtual supervisor trap value
            0x244 => Ok(CSRRegisterName::vsip), // virtual supervisor interrupt pending
            0x280 => Ok(CSRRegisterName::vsatp), // virtual supervisor address translation and protection

            // virtual supervisor indirect
            0x250 => Ok(CSRRegisterName::vsiselect), // virtual supervisor indirect register select
            0x251 => Ok(CSRRegisterName::vsireg), // virtual supervisor indirect register alias
            0x252 => Ok(CSRRegisterName::vsireg2), // virtual supervisor indirect register alias 2
            0x253 => Ok(CSRRegisterName::vsireg3), // virtual supervisor indirect register alias 3
            0x255 => Ok(CSRRegisterName::vsireg4), // virtual supervisor indirect register alias 4
            0x256 => Ok(CSRRegisterName::vsireg5), // virtual supervisor indirect register alias 5
            0x257 => Ok(CSRRegisterName::vsireg6), // virtual supervisor indirect register alias 6

            // virtual supervisor timer compare
            0x24D => Ok(CSRRegisterName::vstimecpm), // virtual supervisor timer compare
            0x25D => Ok(CSRRegisterName::vstimecpmh), // virtual upper 32 bits of `vstimecpm`, RV32 only

            // virtual supervisor control transfer records configuration
            0x24E => Ok(CSRRegisterName::vsctrctl), // virtual supervisor control transfer records control register

            // machine-level CSRs

            // machine information registers
            0xF11 => Ok(CSRRegisterName::mvendorid), // vendor ID
            0xF12 => Ok(CSRRegisterName::marchid),   // architecture ID
            0xF13 => Ok(CSRRegisterName::mimpid),    // implementation ID
            0xF14 => Ok(CSRRegisterName::mhartid),   // hardware thread ID
            0xF15 => Ok(CSRRegisterName::mconfigptr), // pointer to configuration data structure

            // machine trap setup
            0x300 => Ok(CSRRegisterName::mstatus), // machine status register
            0x301 => Ok(CSRRegisterName::misa),    // ISA and extensions
            0x302 => Ok(CSRRegisterName::medeleg), // machine exception delegation register
            0x303 => Ok(CSRRegisterName::mideleg), // machine interrupt delegation register
            0x304 => Ok(CSRRegisterName::mie), // machine interrupt-enable register
            0x305 => Ok(CSRRegisterName::mtvec), // machine trap-handler base address
            0x306 => Ok(CSRRegisterName::mcounteren), // machine counter enable
            0x310 => Ok(CSRRegisterName::mstatush), // 32 upper bits of `mstatus`, RV32 only
            0x312 => Ok(CSRRegisterName::medelegh), // 32 upper bits of `medeleg`, RV32 only

            // machine trap handling
            0x340 => Ok(CSRRegisterName::mscratch), // machine scratch register
            0x341 => Ok(CSRRegisterName::mepc), // machine exception program counter
            0x342 => Ok(CSRRegisterName::mcause), // machine trap cause
            0x343 => Ok(CSRRegisterName::mtval), // machine trap value
            0x344 => Ok(CSRRegisterName::mip),  // machine interrupt pending
            0x34A => Ok(CSRRegisterName::mtinst), // machine
            0x34B => Ok(CSRRegisterName::mtval2), // machine second trap value

            // machine indirect
            0x350 => Ok(CSRRegisterName::miselect), // machine indirect register select
            0x351 => Ok(CSRRegisterName::mireg), // machine indirect register alias
            0x352 => Ok(CSRRegisterName::mireg2), // machine indirect register alias 2
            0x353 => Ok(CSRRegisterName::mireg3), // machine indirect register alias 3
            0x355 => Ok(CSRRegisterName::mireg4), // machine indirect register alias 4
            0x356 => Ok(CSRRegisterName::mireg5), // machine indirect register alias 5
            0x357 => Ok(CSRRegisterName::mireg6), // machine indirect register alias 6

            // machine configuration
            0x30A => Ok(CSRRegisterName::menvcfg), // machine environment configuration register
            0x31A => Ok(CSRRegisterName::menvcfgh), // 32 upper bits of `menvcfg`, RV32 only
            0x747 => Ok(CSRRegisterName::mseccfg), // machine security configuration register
            0x757 => Ok(CSRRegisterName::mseccfgh), // 32 upper bits of `mseccfg`, RV32 only

            // machine memory protection
            0x3A0 => Ok(CSRRegisterName::pmpcfg0), // physical memory protection configuraiton
            0x3A1 => Ok(CSRRegisterName::pmpcfg1), // physical memory protection configuraiton, RV32 only
            0x3A2 => Ok(CSRRegisterName::pmpcfg2), // physical memory protection configuraiton
            0x3A3 => Ok(CSRRegisterName::pmpcfg3), // physical memory protection configuraiton, RV32 only
            0x3A4 => Ok(CSRRegisterName::pmpcfg4), // physical memory protection configuraiton
            0x3A5 => Ok(CSRRegisterName::pmpcfg5), // physical memory protection configuraiton, RV32 only
            0x3A6 => Ok(CSRRegisterName::pmpcfg6), // physical memory protection configuraiton
            0x3A7 => Ok(CSRRegisterName::pmpcfg7), // physical memory protection configuraiton, RV32 only
            0x3A8 => Ok(CSRRegisterName::pmpcfg8), // physical memory protection configuraiton
            0x3A9 => Ok(CSRRegisterName::pmpcfg9), // physical memory protection configuraiton, RV32 only
            0x3AA => Ok(CSRRegisterName::pmpcfg10), // physical memory protection configuraiton
            0x3AB => Ok(CSRRegisterName::pmpcfg11), // physical memory protection configuraiton, RV32 only
            0x3AC => Ok(CSRRegisterName::pmpcfg12), // physical memory protection configuraiton
            0x3AD => Ok(CSRRegisterName::pmpcfg13), // physical memory protection configuraiton, RV32 only
            0x3AE => Ok(CSRRegisterName::pmpcfg14), // physical memory protection configuraiton
            0x3AF => Ok(CSRRegisterName::pmpcfg15), // physical memory protection configuraiton, RV32 only
            0x3B0 => Ok(CSRRegisterName::pmaddr0), // physical memory protection address register
            0x3B1 => Ok(CSRRegisterName::pmaddr1), // physical memory protection address register
            0x3B2 => Ok(CSRRegisterName::pmaddr2), // physical memory protection address register
            0x3B3 => Ok(CSRRegisterName::pmaddr3), // physical memory protection address register
            0x3B4 => Ok(CSRRegisterName::pmaddr4), // physical memory protection address register
            0x3B5 => Ok(CSRRegisterName::pmaddr5), // physical memory protection address register
            0x3B6 => Ok(CSRRegisterName::pmaddr6), // physical memory protection address register
            0x3B7 => Ok(CSRRegisterName::pmaddr7), // physical memory protection address register
            0x3B8 => Ok(CSRRegisterName::pmaddr8), // physical memory protection address register
            0x3B9 => Ok(CSRRegisterName::pmaddr9), // physical memory protection address register
            0x3BA => Ok(CSRRegisterName::pmaddr10), // physical memory protection address register
            0x3BB => Ok(CSRRegisterName::pmaddr11), // physical memory protection address register
            0x3BC => Ok(CSRRegisterName::pmaddr12), // physical memory protection address register
            0x3BD => Ok(CSRRegisterName::pmaddr13), // physical memory protection address register
            0x3BE => Ok(CSRRegisterName::pmaddr14), // physical memory protection address register
            0x3BF => Ok(CSRRegisterName::pmaddr15), // physical memory protection address register
            0x3C0 => Ok(CSRRegisterName::pmaddr16), // physical memory protection address register
            0x3C1 => Ok(CSRRegisterName::pmaddr17), // physical memory protection address register
            0x3C2 => Ok(CSRRegisterName::pmaddr18), // physical memory protection address register
            0x3C3 => Ok(CSRRegisterName::pmaddr19), // physical memory protection address register
            0x3C4 => Ok(CSRRegisterName::pmaddr20), // physical memory protection address register
            0x3C5 => Ok(CSRRegisterName::pmaddr21), // physical memory protection address register
            0x3C6 => Ok(CSRRegisterName::pmaddr22), // physical memory protection address register
            0x3C7 => Ok(CSRRegisterName::pmaddr23), // physical memory protection address register
            0x3C8 => Ok(CSRRegisterName::pmaddr24), // physical memory protection address register
            0x3C9 => Ok(CSRRegisterName::pmaddr25), // physical memory protection address register
            0x3CA => Ok(CSRRegisterName::pmaddr26), // physical memory protection address register
            0x3CB => Ok(CSRRegisterName::pmaddr27), // physical memory protection address register
            0x3CC => Ok(CSRRegisterName::pmaddr28), // physical memory protection address register
            0x3CD => Ok(CSRRegisterName::pmaddr29), // physical memory protection address register
            0x3CE => Ok(CSRRegisterName::pmaddr30), // physical memory protection address register
            0x3CF => Ok(CSRRegisterName::pmaddr31), // physical memory protection address register
            0x3D0 => Ok(CSRRegisterName::pmaddr32), // physical memory protection address register
            0x3D1 => Ok(CSRRegisterName::pmaddr33), // physical memory protection address register
            0x3D2 => Ok(CSRRegisterName::pmaddr34), // physical memory protection address register
            0x3D3 => Ok(CSRRegisterName::pmaddr35), // physical memory protection address register
            0x3D4 => Ok(CSRRegisterName::pmaddr36), // physical memory protection address register
            0x3D5 => Ok(CSRRegisterName::pmaddr37), // physical memory protection address register
            0x3D6 => Ok(CSRRegisterName::pmaddr38), // physical memory protection address register
            0x3D7 => Ok(CSRRegisterName::pmaddr39), // physical memory protection address register
            0x3D8 => Ok(CSRRegisterName::pmaddr40), // physical memory protection address register
            0x3D9 => Ok(CSRRegisterName::pmaddr41), // physical memory protection address register
            0x3DA => Ok(CSRRegisterName::pmaddr42), // physical memory protection address register
            0x3DB => Ok(CSRRegisterName::pmaddr43), // physical memory protection address register
            0x3DC => Ok(CSRRegisterName::pmaddr44), // physical memory protection address register
            0x3DD => Ok(CSRRegisterName::pmaddr45), // physical memory protection address register
            0x3DE => Ok(CSRRegisterName::pmaddr46), // physical memory protection address register
            0x3DF => Ok(CSRRegisterName::pmaddr47), // physical memory protection address register
            0x3E0 => Ok(CSRRegisterName::pmaddr48), // physical memory protection address register
            0x3E1 => Ok(CSRRegisterName::pmaddr49), // physical memory protection address register
            0x3E2 => Ok(CSRRegisterName::pmaddr50), // physical memory protection address register
            0x3E3 => Ok(CSRRegisterName::pmaddr51), // physical memory protection address register
            0x3E4 => Ok(CSRRegisterName::pmaddr52), // physical memory protection address register
            0x3E5 => Ok(CSRRegisterName::pmaddr53), // physical memory protection address register
            0x3E6 => Ok(CSRRegisterName::pmaddr54), // physical memory protection address register
            0x3E7 => Ok(CSRRegisterName::pmaddr55), // physical memory protection address register
            0x3E8 => Ok(CSRRegisterName::pmaddr56), // physical memory protection address register
            0x3E9 => Ok(CSRRegisterName::pmaddr57), // physical memory protection address register
            0x3EA => Ok(CSRRegisterName::pmaddr58), // physical memory protection address register
            0x3EB => Ok(CSRRegisterName::pmaddr59), // physical memory protection address register
            0x3EC => Ok(CSRRegisterName::pmaddr60), // physical memory protection address register
            0x3ED => Ok(CSRRegisterName::pmaddr61), // physical memory protection address register
            0x3EE => Ok(CSRRegisterName::pmaddr62), // physical memory protection address register
            0x3EF => Ok(CSRRegisterName::pmaddr63), // physical memory protection address register

            // machine state enable registers
            0x30C => Ok(CSRRegisterName::mstateen0), // machine state enable 0 register
            0x30D => Ok(CSRRegisterName::mstateen1), // machine state enable 1 register
            0x30E => Ok(CSRRegisterName::mstateen2), // machine state enable 2 register
            0x30F => Ok(CSRRegisterName::mstateen3), // machine state enable 3 register
            0x31C => Ok(CSRRegisterName::mstateen0h), // 32 upper bits of `mstateen0`, RV32 only
            0x31D => Ok(CSRRegisterName::mstateen1h), // 32 upper bits of `mstateen1`, RV32 only
            0x31E => Ok(CSRRegisterName::mstateen2h), // 32 upper bits of `mstateen2`, RV32 only
            0x31F => Ok(CSRRegisterName::mstateen3h), // 32 upper bits of `mstateen3`, RV32 only

            // machine non-maskable interrupt handling
            0x740 => Ok(CSRRegisterName::mnscratch), // resumable NMI scratch register
            0x741 => Ok(CSRRegisterName::mnepc), // resumable NMI program counter
            0x742 => Ok(CSRRegisterName::mncause), // resumable NMI cause
            0x744 => Ok(CSRRegisterName::mnstatus), // resumable NMI status

            // machine counter/timers
            0xB00 => Ok(CSRRegisterName::mcycle), // machine cycle counter
            0xB02 => Ok(CSRRegisterName::minstret), // machine instructions-retired counter
            0xB03 => Ok(CSRRegisterName::mhpmcounter3), // machine performance-monitoring counter
            0xB04 => Ok(CSRRegisterName::mhpmcounter4), // machine performance-monitoring counter
            0xB05 => Ok(CSRRegisterName::mhpmcounter5), // machine performance-monitoring counter
            0xB06 => Ok(CSRRegisterName::mhpmcounter6), // machine performance-monitoring counter
            0xB07 => Ok(CSRRegisterName::mhpmcounter7), // machine performance-monitoring counter
            0xB08 => Ok(CSRRegisterName::mhpmcounter8), // machine performance-monitoring counter
            0xB09 => Ok(CSRRegisterName::mhpmcounter9), // machine performance-monitoring counter
            0xB0A => Ok(CSRRegisterName::mhpmcounter10), // machine performance-monitoring counter
            0xB0B => Ok(CSRRegisterName::mhpmcounter11), // machine performance-monitoring counter
            0xB0C => Ok(CSRRegisterName::mhpmcounter12), // machine performance-monitoring counter
            0xB0D => Ok(CSRRegisterName::mhpmcounter13), // machine performance-monitoring counter
            0xB0E => Ok(CSRRegisterName::mhpmcounter14), // machine performance-monitoring counter
            0xB0F => Ok(CSRRegisterName::mhpmcounter15), // machine performance-monitoring counter
            0xB10 => Ok(CSRRegisterName::mhpmcounter16), // machine performance-monitoring counter
            0xB11 => Ok(CSRRegisterName::mhpmcounter17), // machine performance-monitoring counter
            0xB12 => Ok(CSRRegisterName::mhpmcounter18), // machine performance-monitoring counter
            0xB13 => Ok(CSRRegisterName::mhpmcounter19), // machine performance-monitoring counter
            0xB14 => Ok(CSRRegisterName::mhpmcounter20), // machine performance-monitoring counter
            0xB15 => Ok(CSRRegisterName::mhpmcounter21), // machine performance-monitoring counter
            0xB16 => Ok(CSRRegisterName::mhpmcounter22), // machine performance-monitoring counter
            0xB17 => Ok(CSRRegisterName::mhpmcounter23), // machine performance-monitoring counter
            0xB18 => Ok(CSRRegisterName::mhpmcounter24), // machine performance-monitoring counter
            0xB19 => Ok(CSRRegisterName::mhpmcounter25), // machine performance-monitoring counter
            0xB1A => Ok(CSRRegisterName::mhpmcounter26), // machine performance-monitoring counter
            0xB1B => Ok(CSRRegisterName::mhpmcounter27), // machine performance-monitoring counter
            0xB1C => Ok(CSRRegisterName::mhpmcounter28), // machine performance-monitoring counter
            0xB1D => Ok(CSRRegisterName::mhpmcounter29), // machine performance-monitoring counter
            0xB1E => Ok(CSRRegisterName::mhpmcounter30), // machine performance-monitoring counter
            0xB1F => Ok(CSRRegisterName::mhpmcounter31), // machine performance-monitoring counter

            0xB80 => Ok(CSRRegisterName::mcycleh), // upper 32 bits of `mcycle`, RV32 only
            0xB82 => Ok(CSRRegisterName::minstreth), // upper 32 bits of `minstret`, RV32 only
            0xB83 => Ok(CSRRegisterName::mhpmcounter3h), // upper 32 bits of `mhpmcounter3`, RV32 only
            0xB84 => Ok(CSRRegisterName::mhpmcounter4h), // upper 32 bits of `mhpmcounter4`, RV32 only
            0xB85 => Ok(CSRRegisterName::mhpmcounter5h), // upper 32 bits of `mhpmcounter5`, RV32 only
            0xB86 => Ok(CSRRegisterName::mhpmcounter6h), // upper 32 bits of `mhpmcounter6`, RV32 only
            0xB87 => Ok(CSRRegisterName::mhpmcounter7h), // upper 32 bits of `mhpmcounter7`, RV32 only
            0xB88 => Ok(CSRRegisterName::mhpmcounter8h), // upper 32 bits of `mhpmcounter8`, RV32 only
            0xB89 => Ok(CSRRegisterName::mhpmcounter9h), // upper 32 bits of `mhpmcounter9`, RV32 only
            0xB8A => Ok(CSRRegisterName::mhpmcounter10h), // upper 32 bits of `mhpmcounter10`, RV32 only
            0xB8B => Ok(CSRRegisterName::mhpmcounter11h), // upper 32 bits of `mhpmcounter11`, RV32 only
            0xB8C => Ok(CSRRegisterName::mhpmcounter12h), // upper 32 bits of `mhpmcounter12`, RV32 only
            0xB8D => Ok(CSRRegisterName::mhpmcounter13h), // upper 32 bits of `mhpmcounter13`, RV32 only
            0xB8E => Ok(CSRRegisterName::mhpmcounter14h), // upper 32 bits of `mhpmcounter14`, RV32 only
            0xB8F => Ok(CSRRegisterName::mhpmcounter15h), // upper 32 bits of `mhpmcounter15`, RV32 only
            0xB90 => Ok(CSRRegisterName::mhpmcounter16h), // upper 32 bits of `mhpmcounter16`, RV32 only
            0xB91 => Ok(CSRRegisterName::mhpmcounter17h), // upper 32 bits of `mhpmcounter17`, RV32 only
            0xB92 => Ok(CSRRegisterName::mhpmcounter18h), // upper 32 bits of `mhpmcounter18`, RV32 only
            0xB93 => Ok(CSRRegisterName::mhpmcounter19h), // upper 32 bits of `mhpmcounter19`, RV32 only
            0xB94 => Ok(CSRRegisterName::mhpmcounter20h), // upper 32 bits of `mhpmcounter20`, RV32 only
            0xB95 => Ok(CSRRegisterName::mhpmcounter21h), // upper 32 bits of `mhpmcounter21`, RV32 only
            0xB96 => Ok(CSRRegisterName::mhpmcounter22h), // upper 32 bits of `mhpmcounter22`, RV32 only
            0xB97 => Ok(CSRRegisterName::mhpmcounter23h), // upper 32 bits of `mhpmcounter23`, RV32 only
            0xB98 => Ok(CSRRegisterName::mhpmcounter24h), // upper 32 bits of `mhpmcounter24`, RV32 only
            0xB99 => Ok(CSRRegisterName::mhpmcounter25h), // upper 32 bits of `mhpmcounter25`, RV32 only
            0xB9A => Ok(CSRRegisterName::mhpmcounter26h), // upper 32 bits of `mhpmcounter26`, RV32 only
            0xB9B => Ok(CSRRegisterName::mhpmcounter27h), // upper 32 bits of `mhpmcounter27`, RV32 only
            0xB9C => Ok(CSRRegisterName::mhpmcounter28h), // upper 32 bits of `mhpmcounter28`, RV32 only
            0xB9D => Ok(CSRRegisterName::mhpmcounter29h), // upper 32 bits of `mhpmcounter29`, RV32 only
            0xB9E => Ok(CSRRegisterName::mhpmcounter30h), // upper 32 bits of `mhpmcounter30`, RV32 only
            0xB9F => Ok(CSRRegisterName::mhpmcounter31h), // upper 32 bits of `mhpmcounter31`, RV32 only

            // machine counter setup
            0x320 => Ok(CSRRegisterName::mcountinhibit), // machine counter-inhibit register
            0x321 => Ok(CSRRegisterName::mcyclecfg), // machine cycle-counter configuration register
            0x322 => Ok(CSRRegisterName::minstretcfg), // machine insert counter configuration register
            0x323 => Ok(CSRRegisterName::mhpmevent3), // machine performance-monitoring event selector
            0x324 => Ok(CSRRegisterName::mhpmevent4), // machine performance-monitoring event selector
            0x325 => Ok(CSRRegisterName::mhpmevent5), // machine performance-monitoring event selector
            0x326 => Ok(CSRRegisterName::mhpmevent6), // machine performance-monitoring event selector
            0x327 => Ok(CSRRegisterName::mhpmevent7), // machine performance-monitoring event selector
            0x328 => Ok(CSRRegisterName::mhpmevent8), // machine performance-monitoring event selector
            0x329 => Ok(CSRRegisterName::mhpmevent9), // machine performance-monitoring event selector
            0x32A => Ok(CSRRegisterName::mhpmevent10), // machine performance-monitoring event selector
            0x32B => Ok(CSRRegisterName::mhpmevent11), // machine performance-monitoring event selector
            0x32C => Ok(CSRRegisterName::mhpmevent12), // machine performance-monitoring event selector
            0x32D => Ok(CSRRegisterName::mhpmevent13), // machine performance-monitoring event selector
            0x32E => Ok(CSRRegisterName::mhpmevent14), // machine performance-monitoring event selector
            0x32F => Ok(CSRRegisterName::mhpmevent15), // machine performance-monitoring event selector
            0x330 => Ok(CSRRegisterName::mhpmevent16), // machine performance-monitoring event selector
            0x331 => Ok(CSRRegisterName::mhpmevent17), // machine performance-monitoring event selector
            0x332 => Ok(CSRRegisterName::mhpmevent18), // machine performance-monitoring event selector
            0x333 => Ok(CSRRegisterName::mhpmevent19), // machine performance-monitoring event selector
            0x334 => Ok(CSRRegisterName::mhpmevent20), // machine performance-monitoring event selector
            0x335 => Ok(CSRRegisterName::mhpmevent21), // machine performance-monitoring event selector
            0x336 => Ok(CSRRegisterName::mhpmevent22), // machine performance-monitoring event selector
            0x337 => Ok(CSRRegisterName::mhpmevent23), // machine performance-monitoring event selector
            0x338 => Ok(CSRRegisterName::mhpmevent24), // machine performance-monitoring event selector
            0x339 => Ok(CSRRegisterName::mhpmevent25), // machine performance-monitoring event selector
            0x33A => Ok(CSRRegisterName::mhpmevent26), // machine performance-monitoring event selector
            0x33B => Ok(CSRRegisterName::mhpmevent27), // machine performance-monitoring event selector
            0x33C => Ok(CSRRegisterName::mhpmevent28), // machine performance-monitoring event selector
            0x33D => Ok(CSRRegisterName::mhpmevent29), // machine performance-monitoring event selector
            0x33E => Ok(CSRRegisterName::mhpmevent30), // machine performance-monitoring event selector
            0x33F => Ok(CSRRegisterName::mhpmevent31), // machine performance-monitoring event selector

            0x721 => Ok(CSRRegisterName::mcyclecfgh), // upper 32 bits of `mcyclecfg`, RV32 only
            0x722 => Ok(CSRRegisterName::minstretcfgh), // upper 32 bits of `minstretcfg`, RV32 only
            0x723 => Ok(CSRRegisterName::mhpmevent3h), // upper 32 bits of `mhpmevent3`, RV32 only
            0x724 => Ok(CSRRegisterName::mhpmevent4h), // upper 32 bits of `mhpmevent4`, RV32 only
            0x725 => Ok(CSRRegisterName::mhpmevent5h), // upper 32 bits of `mhpmevent5`, RV32 only
            0x726 => Ok(CSRRegisterName::mhpmevent6h), // upper 32 bits of `mhpmevent6`, RV32 only
            0x727 => Ok(CSRRegisterName::mhpmevent7h), // upper 32 bits of `mhpmevent7`, RV32 only
            0x728 => Ok(CSRRegisterName::mhpmevent8h), // upper 32 bits of `mhpmevent8`, RV32 only
            0x729 => Ok(CSRRegisterName::mhpmevent9h), // upper 32 bits of `mhpmevent9`, RV32 only
            0x72A => Ok(CSRRegisterName::mhpmevent10h), // upper 32 bits of `mhpmevent10`, RV32 only
            0x72B => Ok(CSRRegisterName::mhpmevent11h), // upper 32 bits of `mhpmevent11`, RV32 only
            0x72C => Ok(CSRRegisterName::mhpmevent12h), // upper 32 bits of `mhpmevent12`, RV32 only
            0x72D => Ok(CSRRegisterName::mhpmevent13h), // upper 32 bits of `mhpmevent13`, RV32 only
            0x72E => Ok(CSRRegisterName::mhpmevent14h), // upper 32 bits of `mhpmevent14`, RV32 only
            0x72F => Ok(CSRRegisterName::mhpmevent15h), // upper 32 bits of `mhpmevent15`, RV32 only
            0x730 => Ok(CSRRegisterName::mhpmevent16h), // upper 32 bits of `mhpmevent16`, RV32 only
            0x731 => Ok(CSRRegisterName::mhpmevent17h), // upper 32 bits of `mhpmevent17`, RV32 only
            0x732 => Ok(CSRRegisterName::mhpmevent18h), // upper 32 bits of `mhpmevent18`, RV32 only
            0x733 => Ok(CSRRegisterName::mhpmevent19h), // upper 32 bits of `mhpmevent19`, RV32 only
            0x734 => Ok(CSRRegisterName::mhpmevent20h), // upper 32 bits of `mhpmevent20`, RV32 only
            0x735 => Ok(CSRRegisterName::mhpmevent21h), // upper 32 bits of `mhpmevent21`, RV32 only
            0x736 => Ok(CSRRegisterName::mhpmevent22h), // upper 32 bits of `mhpmevent22`, RV32 only
            0x737 => Ok(CSRRegisterName::mhpmevent23h), // upper 32 bits of `mhpmevent23`, RV32 only
            0x738 => Ok(CSRRegisterName::mhpmevent24h), // upper 32 bits of `mhpmevent24`, RV32 only
            0x739 => Ok(CSRRegisterName::mhpmevent25h), // upper 32 bits of `mhpmevent25`, RV32 only
            0x73A => Ok(CSRRegisterName::mhpmevent26h), // upper 32 bits of `mhpmevent26`, RV32 only
            0x73B => Ok(CSRRegisterName::mhpmevent27h), // upper 32 bits of `mhpmevent27`, RV32 only
            0x73C => Ok(CSRRegisterName::mhpmevent28h), // upper 32 bits of `mhpmevent28`, RV32 only
            0x73D => Ok(CSRRegisterName::mhpmevent29h), // upper 32 bits of `mhpmevent29`, RV32 only
            0x73E => Ok(CSRRegisterName::mhpmevent30h), // upper 32 bits of `mhpmevent30`, RV32 only
            0x73F => Ok(CSRRegisterName::mhpmevent31h), // upper 32 bits of `mhpmevent31`, RV32 only

            // machine control transfer records configuration
            0x34E => Ok(CSRRegisterName::mctrctl), // machine control transfer records control register

            // machine/debug level CSRs

            // debug/trace registers (shared with debug mode)
            0x7A0 => Ok(CSRRegisterName::tselect), // debug/trace trigger register select
            0x7A1 => Ok(CSRRegisterName::tdata1), // first debug/trace trigger data register
            0x7A2 => Ok(CSRRegisterName::tdata2), // second debug/trace trigger data register
            0x7A3 => Ok(CSRRegisterName::tdata3), // third debug/trace trigger data register
            0x7A8 => Ok(CSRRegisterName::mcontext), // machine-mode context register

            // debug mode registers

            // debug mode registers
            0x7B0 => Ok(CSRRegisterName::dcsr), // debug control and status register
            0x7B1 => Ok(CSRRegisterName::dpc),  // debug program counter
            0x7B2 => Ok(CSRRegisterName::dscratch0), // debug scratch register 0
            0x7B3 => Ok(CSRRegisterName::dscratch1), // debug scratch register 1
            //
            _ => Err("unknown CSR offset"),
        }
    }
}

impl Display for CSRRegisterName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let csr_name_string = "{self:?}";
        // match self {
        //     //CSRRegisterName::fflags => csr_name_string = "self:#?",

        //     // _ => csr_name_string = "unknown CSR: {self:#}",
        // }

        write!(formatter, "{csr_name_string}")
    }
}
