extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};

use std::collections::HashMap;

pub struct Elf {
    e_hdr: Elf64Ehdr,
    p_hdr: Vec<Elf64Phdr>,
    program: Vec<u8>,
    data: Vec<Vec<u8>>,
    shstrtab: Vec<u8>,
    s_hdr: Vec<Elf64Shdr>,
}

impl Elf {
    // data is a list of constants
    // rewrites is a map of indexes into the program to data indexes
    pub fn new(mut program: Vec<u8>, data: Vec<Vec<u8>>, rewrites: HashMap<usize, usize>) -> Self {
        let shstrtab = b"\0.text\0.data\0.shstrtab\0";
        let data_offset = 64+56+56+program.len() as u64;

        // Read memory positions of each data entry
        let mut data_position = Vec::new();
        let mut pos = DATA_LOCATION + data_offset;
        let mut data_len = 0;
        for id in &data {
            data_position.push(pos);
            pos += id.len() as u64;
            data_len += id.len();
        }
        // Perform rewrites
        for (p, i) in rewrites {
            (&mut program[p..]).write_u64::<LittleEndian>(data_position[i]).unwrap();
        }

        let p_hdr_size = data_offset;
        let shstrtab_offset = data_offset + data_len as u64;
        let sh_off = shstrtab_offset + shstrtab.len() as u64;
        Elf {
            e_hdr: Elf64Ehdr::new(sh_off),
            p_hdr: vec![Elf64Phdr::text(0, p_hdr_size),
                        Elf64Phdr::data(data_offset, data_len as u64)],
            shstrtab: shstrtab.to_vec(),
            s_hdr: vec![Elf64Shdr::null(), Elf64Shdr::text(program.len() as u64),
                        Elf64Shdr::data(data_len as u64, data_offset),
                        Elf64Shdr::shstrtab(shstrtab.len() as u64, shstrtab_offset)],
            data: data,
            program: program,
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        let Elf { e_hdr, p_hdr, mut program, data, mut shstrtab, s_hdr } = self;

        let mut v = Vec::new();
        v.append(&mut e_hdr.to_vec());
        for p in p_hdr {
            v.append(&mut p.to_vec());
        }
        v.append(&mut program);
        for mut d in data {
            v.append(&mut d);
        }
        v.append(&mut shstrtab);
        for s in s_hdr {
            v.append(&mut s.to_vec());
        }
        v
    }
}

type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64Xword = u64;

const EI_NIDENT: usize = 16; // Number of bytes in e_ident
const DATA_LOCATION: u64 = 0x600000;

struct Elf64Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}

impl Elf64Ehdr {
    fn new(sh_off: u64) -> Self {
        Elf64Ehdr {
            e_ident: [0x7f, b'E', b'L', b'F', // magic number
                      2, // 1 for 32 bit, 2 for 64bit
                      1, // endianness - 1 is little-endian
                      1, // version
                      0, // abi version
                      0, 0, 0, 0, 0, 0, 0, 0], // unused padding
            // object file type: 2 if executable
            e_type: 2,
            // target isa: 0x3e is amd64
            e_machine: 0x3e,
            // set to 1 for the original version of elf
            e_version: 1,
            // memory address of the entry point. right after elf header + program header
            e_entry: 0x4000b0,
            // phoff: points to start of the program header table
            e_phoff: 0x40,
            // shoff: points to start of the section header table
            //e_shoff: 0x110,
            e_shoff: sh_off,
            // flags
            e_flags: 0,
            // size of this header, usually 64 bytes on 64bit
            e_ehsize: 64,
            // size of a program header table entry
            e_phentsize: 56,
            // number of entries in the program header table
            e_phnum: 2,
            // size of a section header table entry
            e_shentsize: 64,
            // number of entries in the section header table
            e_shnum: 4,
            // index of the section header table entry that contains the section names
            e_shstrndx: 3,
        }
    }

    fn to_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.e_ident);
        v.write_u16::<LittleEndian>(self.e_type).unwrap();
        v.write_u16::<LittleEndian>(self.e_machine).unwrap();
        v.write_u32::<LittleEndian>(self.e_version).unwrap();
        v.write_u64::<LittleEndian>(self.e_entry).unwrap();
        v.write_u64::<LittleEndian>(self.e_phoff).unwrap();
        v.write_u64::<LittleEndian>(self.e_shoff).unwrap();
        v.write_u32::<LittleEndian>(self.e_flags).unwrap();
        v.write_u16::<LittleEndian>(self.e_ehsize).unwrap();
        v.write_u16::<LittleEndian>(self.e_phentsize).unwrap();
        v.write_u16::<LittleEndian>(self.e_phnum).unwrap();
        v.write_u16::<LittleEndian>(self.e_shentsize).unwrap();
        v.write_u16::<LittleEndian>(self.e_shnum).unwrap();
        v.write_u16::<LittleEndian>(self.e_shstrndx).unwrap();
        v
    }
}

struct Elf64Phdr {
    p_type: Elf64Word,
    p_flags: Elf64Word,
    p_offset: Elf64Off,
    p_vaddr: Elf64Addr,
    p_paddr: Elf64Addr,
    p_filesz: Elf64Xword,
    p_memsz: Elf64Xword,
    p_align: Elf64Xword,
}

impl Elf64Phdr {
    fn text(offset: u64, size: u64,) -> Self {
        Elf64Phdr {
            // 1 is PT_LOAD
            p_type: 1,
            // Execute and read permissions
            p_flags: 5,
            // offset from beginning of segments
            p_offset: offset,
            // Initial virtual memory address to load this segment to
            p_vaddr: 0x400000,
            p_paddr: 0x400000,
            p_filesz: size,
            p_memsz: size,
            p_align: 0x200000,
        }
    }

    fn data(offset: u64, size: u64) -> Self {
        Elf64Phdr {
            // 1 is PT_LOAD
            p_type: 1,
            // read and write permissions
            p_flags: 6,
            // offset from beginning of segments
            p_offset: offset,
            // Initial virtual memory address to load this segment to
            p_vaddr: 0x600000 + offset,
            p_paddr: 0x600000 + offset,
            p_filesz: size,
            p_memsz: size,
            p_align: 0x200000,
        }
    }

    fn to_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.write_u32::<LittleEndian>(self.p_type).unwrap();
        v.write_u32::<LittleEndian>(self.p_flags).unwrap();
        v.write_u64::<LittleEndian>(self.p_offset).unwrap();
        v.write_u64::<LittleEndian>(self.p_vaddr).unwrap();
        v.write_u64::<LittleEndian>(self.p_paddr).unwrap();
        v.write_u64::<LittleEndian>(self.p_filesz).unwrap();
        v.write_u64::<LittleEndian>(self.p_memsz).unwrap();
        v.write_u64::<LittleEndian>(self.p_align).unwrap();
        v
    }
}

struct Elf64Shdr {
    sh_name: Elf64Word,
    sh_type: Elf64Word,
    sh_flags: Elf64Xword,
    sh_addr: Elf64Addr,
    sh_offset: Elf64Off,
    sh_size: Elf64Xword,
    sh_link: Elf64Word,
    sh_info: Elf64Word,
    sh_addralign: Elf64Xword,
    sh_entsize: Elf64Xword,
}

impl Elf64Shdr {
    fn null() -> Self {
        Elf64Shdr {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }

    fn text(sh_size: u64) -> Self {
        Elf64Shdr {
            sh_name: 0x01,
            sh_type: 1,
            sh_flags: 6,
            sh_addr: 0x4000b0,
            sh_offset: 0xb0,
            sh_size: sh_size,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0x10,
            sh_entsize: 0,
        }
    }

    fn data(sh_size: u64, sh_offset: u64) -> Self {
        Elf64Shdr {
            sh_name: 0x07,
            sh_type: 1,
            sh_flags: 3,
            sh_addr: DATA_LOCATION + sh_offset,
            sh_offset: sh_offset,
            sh_size: sh_size,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 4,
            sh_entsize: 0,
        }
    }

    fn shstrtab(sh_size: u64, sh_offset: u64) -> Self {
        Elf64Shdr {
            sh_name: 0x0d,
            sh_type: 3,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: sh_offset,
            sh_size: sh_size,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
        }
    }

    fn to_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.write_u32::<LittleEndian>(self.sh_name).unwrap();
        v.write_u32::<LittleEndian>(self.sh_type).unwrap();
        v.write_u64::<LittleEndian>(self.sh_flags).unwrap();
        v.write_u64::<LittleEndian>(self.sh_addr).unwrap();
        v.write_u64::<LittleEndian>(self.sh_offset).unwrap();
        v.write_u64::<LittleEndian>(self.sh_size).unwrap();
        v.write_u32::<LittleEndian>(self.sh_link).unwrap();
        v.write_u32::<LittleEndian>(self.sh_info).unwrap();
        v.write_u64::<LittleEndian>(self.sh_addralign).unwrap();
        v.write_u64::<LittleEndian>(self.sh_entsize).unwrap();
        v
    }
}
