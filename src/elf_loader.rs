use crate::constants::arch::XLen;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;
use std::fs;
use std::path::PathBuf;

pub struct ElfData {
    pub e_entry: XLen,
    pub p_hdrs: Vec<ProgramHeader>,
}

pub fn read_elf(file_path: PathBuf) -> Option<ElfData> {
    let content = fs::read(file_path).unwrap();
    let slice = content.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).ok()?;

    let all_load_phdrs: Vec<_> = file
        .segments()
        .unwrap()
        .iter()
        .filter(|phdr| phdr.p_type == PT_LOAD)
        .collect();

    Some(ElfData {
        e_entry: file.ehdr.e_entry,
        p_hdrs: all_load_phdrs,
    })
}
