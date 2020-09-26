extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};

pub struct Elf {
    e_hdr: Elf64Ehdr,
    p_hdr: Vec<Elf64Phdr>,
    program: Vec<u8>,
    data: Vec<u8>,
    shstrtab: Vec<u8>,
    s_hdr: Vec<Elf64Shdr>,
}

impl Elf {
    pub fn new(program: Vec<u8>, data: Vec<u8>) -> Self {
        const shstrtab: &'static [u8] =  b"\0.text\0.data\0.shstrtab\0";
        let data_offset = 64+56+56+program.len() as u64;
        let p_hdr_size = data_offset;
        let shstrtab_offset = data_offset + data.len() as u64;
        let sh_off = shstrtab_offset + shstrtab.len() as u64;
        Elf {
            e_hdr: Elf64Ehdr::new(sh_off),
            p_hdr: vec![Elf64Phdr::text(0, p_hdr_size),
                        Elf64Phdr::data(data_offset, data.len() as u64)],
            shstrtab: shstrtab.to_vec(),
            s_hdr: vec![Elf64Shdr::null(), Elf64Shdr::text(program.len() as u64),
                        Elf64Shdr::data(data.len() as u64, data_offset),
                        Elf64Shdr::shstrtab(shstrtab.len() as u64, shstrtab_offset)],
            data: data,
            program: program,
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        let Elf { e_hdr, p_hdr, mut program, mut data, mut shstrtab, s_hdr } = self;

        let mut v = Vec::new();
        v.append(&mut e_hdr.to_vec());
        for p in p_hdr {
            v.append(&mut p.to_vec());
        }
        v.append(&mut program);
        v.append(&mut data);
        v.append(&mut shstrtab);
        for s in s_hdr {
            v.append(&mut s.to_vec());
        }
        v
    }
}

//const ELF_MAGIC: [u8; 4] = [0x73, b'E', b'L', b'F'];

type Elf64_Addr = u64;
type Elf64_Off = u64;
type Elf64_Half = u16;
type Elf64_Word = u32;
type Elf64_Sword = u32;
type Elf64_Xword = u64;
type Elf64_Sxword = u64;

const EI_MAG0: usize = 0; // File identification index
const EI_MAG1: usize = 1; // File identification index
const EI_MAG2: usize = 2; // File identification index
const EI_MAG3: usize = 3; // File identification index
const EI_CLASS: usize = 4; // File class
const EI_DATA: usize = 5; // Data encoding
const EI_VERSION: usize = 6; // File version
const EI_OSABI: usize = 7; // OS/ABI identification
const EI_ABIVERSION: usize = 8; // ABI version
const EI_PAD: usize = 9; // Start of padding bytes
const EI_NIDENT: usize = 16; // Number of bytes in e_ident

struct Elf64Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: Elf64_Half,
    e_machine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half,
}

impl Elf64Ehdr {
    fn new(sh_off: u64) -> Self {
        Elf64Ehdr {
            e_ident: [0x7f, b'E', b'L', b'F', // magic number
                      2, // 1 for 32 bit, 2 for 64bit
                      1, // version
                      1, // target operating system abi
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
    p_type: Elf64_Word,
    p_flags: Elf64_Word,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: Elf64_Xword,
    p_memsz: Elf64_Xword,
    p_align: Elf64_Xword,
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
            //p_filesz: 0xd7,
            //p_memsz: 0xd7,
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
            //p_offset: 0xd8,
            // Initial virtual memory address to load this segment to
            p_vaddr: 0x600000 + offset,
            p_paddr: 0x600000 + offset,
            //p_vaddr: 0x6000d8,
            //p_paddr: 0x6000d8,
            p_filesz: size,
            p_memsz: size,
            //p_filesz: 0x0d,
            //p_memsz: 0x0d,
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
    sh_name: Elf64_Word,
    sh_type: Elf64_Word,
    sh_flags: Elf64_Xword,
    sh_addr: Elf64_Addr,
    sh_offset: Elf64_Off,
    sh_size: Elf64_Xword,
    sh_link: Elf64_Word,
    sh_info: Elf64_Word,
    sh_addralign: Elf64_Xword,
    sh_entsize: Elf64_Xword,
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
            //sh_addr: 0x6000d8,
            sh_addr: 0x600000 + sh_offset,
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
