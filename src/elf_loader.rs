use crate::constants::arch::XLen;
use crate::soc::memory_bus::Bus;
// use crate::soc::soc::SoC;
use crate::soc::SoC;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;
// use elf::
use log::error;
use std::fs;
use std::path::PathBuf;

pub struct ElfData {
    pub e_entry: XLen,
    pub p_hdrs: Vec<ProgramHeader>,
}

pub fn load_elf(file_path: PathBuf, soc: &mut SoC) -> Option<ElfData> {
    // dbg!("{:?}", file_path.clone());
    let content = fs::read(file_path).unwrap();
    let slice = content.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).ok()?;

    let all_load_phdrs: Vec<_> = file
        .segments()
        .unwrap()
        .iter()
        .filter(|phdr| phdr.p_type == PT_LOAD)
        .collect();
    // TODO: conditional compilation for test facilities
    let syms = file
        .symbol_table()
        .map_err(|err| {
            error!("failed to read symbol table: {err}");
            err
        })
        .ok()
        .flatten();
    if let Some((symtab, strtab)) = syms {
        for sym in symtab {
            let sym_name = strtab.get(sym.st_name as usize).ok()?;
            if sym_name == "tohost" {
                // dbg!("GREPME tohost found");
                soc.set_htif(sym.st_value);
            }
        }
    }
    // dbg!("GREPMESEC {:?}", syms);

    let phdrs = all_load_phdrs.iter().filter(|hdr| hdr.p_type == PT_LOAD);
    for phdr in phdrs {
        // dbg!("GREPME {:?}", phdr);
        let mut start_addr = phdr.p_paddr;
        for offset in phdr.p_offset..phdr.p_offset + phdr.p_filesz {
            soc.bus.write(
                {
                    let tmp = start_addr;
                    start_addr += 1;
                    tmp
                },
                content[offset as usize],
            );
        }
    }
    soc.cpu.set_pc(file.ehdr.e_entry);

    // dbg!("{}", file.ehdr.e_entry);
    // dbg!("ELF segments so far: {:?}", all_load_phdrs.clone());

    Some(ElfData {
        e_entry: file.ehdr.e_entry,
        p_hdrs: all_load_phdrs,
    })
}
