use crate::constants::arch::XLen;
use crate::soc::SoC;
use crate::soc::memory_bus::Bus;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;
use log::error;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

use crate::globals::exit_codes::ExitCode;

pub struct ElfData {
    pub e_entry: XLen,
    pub p_hdrs: Vec<ProgramHeader>,
}

pub(crate) fn load_elf(
    file_path: PathBuf,
    soc: &mut SoC,
) -> Result<ElfData, ElfError> {
    let content = fs::read(file_path).map_err(|err| match err {
        _ => ElfError::Read(err),
    })?;
    let slice = content.as_slice();
    let file =
        ElfBytes::<AnyEndian>::minimal_parse(slice).map_err(
            |err| match err {
                _ => ElfError::ParseHeader(err),
            },
        )?;

    let all_load_phdrs = file.segments().ok_or(ElfError::SegmentsLoad)?;
    let all_load_phdrs: Vec<_> = all_load_phdrs
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
            let sym_name =
                strtab.get(sym.st_name as usize).map_err(|err| match err {
                    _ => ElfError::StringTable(err),
                })?;
            if sym_name == "tohost" {
                soc.set_htif(sym.st_value);
            }
        }
    }

    let phdrs = all_load_phdrs.iter().filter(|hdr| hdr.p_type == PT_LOAD);
    for phdr in phdrs {
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
    for hart in soc.harts.iter_mut() {
        hart.set_pc(file.ehdr.e_entry);
    }

    Ok(ElfData {
        e_entry: file.ehdr.e_entry,
        p_hdrs: all_load_phdrs,
    })
}

#[derive(Debug)]
pub enum ElfError {
    Read(std::io::Error),
    ParseHeader(elf::ParseError),
    SegmentsLoad,
    StringTable(elf::ParseError),
}

impl ElfError {
    pub fn exit_code(&self) -> i32 {
        match self {
            ElfError::Read(_) => ExitCode::ElfRead as i32,
            ElfError::ParseHeader(_) => ExitCode::ElfParseHeader as i32,
            ElfError::SegmentsLoad => ExitCode::SegmentsLoad as i32,
            ElfError::StringTable(_) => ExitCode::StringTable as i32,
        }
    }
}

impl Display for ElfError {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            ElfError::Read(err) => {
                write!(formatter, "{err}")
            }
            ElfError::ParseHeader(err) => {
                write!(formatter, "{err}")
            }
            ElfError::SegmentsLoad => {
                write!(formatter, "failed to load segments")
            }
            ElfError::StringTable(err) => {
                write!(formatter, "{err}")
            }
        }
    }
}

impl Error for ElfError {}
