//use byteorder::{LittleEndian, WriteBytesExt};

use std::fs::File;
use std::io::Write;

pub fn generate_executable(path: String, machine_code: Vec<u8>) {
    /*
    let code_len = machine_code.len();
    let mut elf = Vec::new();
    write_elf_header(&mut elf, code_len as u64);
    write_program_header(&mut elf);
    elf.append(&mut machine_code);
    write_section_header(&mut elf, code_len);

    let mut file = File::create(path).unwrap();
    file.write_all(&elf).unwrap();
    */
    let mut file = File::create(path).unwrap();
    file.write_all(&machine_code).unwrap();
}

/*
fn write_elf_header(elf: &mut Vec<u8>, code_len: u64) {
    // magic number
    elf.push(0x7F);
    elf.push(b'E');
    elf.push(b'L');
    elf.push(b'F');
    // 1 for 32bit, 2 for 64bit
    elf.push(2);
    // 1 for little endian, 2 for big endian
    elf.push(1);
    // version
    elf.push(1);
    // target operating system abi
    elf.push(0);
    // abi version
    elf.push(0);
    // unused padding
    for _ in 0..7 {
        elf.push(0);
    }
    // object file type: 2 is executable
    elf.write_u16::<LittleEndian>(2).unwrap();
    //elf.push(2);
    //elf.push(0);
    // target isa: 0x3e is amd64
    elf.write_u16::<LittleEndian>(0x3E).unwrap();
    //elf.push(0x3E);
    //elf.push(0);
    // set to 1 for the orignal version of elf
    elf.write_u32::<LittleEndian>(1).unwrap();
    //elf.push(1);
    //elf.push(0);
    //elf.push(0);
    //elf.push(0);

    // memory address of the entry point. right after elf header + program header
    elf.write_u64::<LittleEndian>(64 + 56).unwrap();
    // phoff: points to start of the program header table
    elf.write_u64::<LittleEndian>(64).unwrap();
    // shoff: points to start of the section header table
    elf.write_u64::<LittleEndian>(64 + 56 + code_len).unwrap();
    // flags
    elf.write_u32::<LittleEndian>(0).unwrap();
    /*for _ in 0..4 {
        elf.push(0);
    }*/
    // size of this header, usually 64 bytes on 64bit
    elf.write_u16::<LittleEndian>(64).unwrap();
    //elf.push(64);
    //elf.push(0);
    // size of a program header table entry
    elf.write_u16::<LittleEndian>(56).unwrap();
    //elf.push(56);
    //elf.push(0);
    // number of entries in the program header table
    elf.write_u16::<LittleEndian>(1).unwrap();
    //elf.push(1);
    //elf.push(0);
    // size of a section header table entry
    elf.write_u16::<LittleEndian>(64).unwrap();
    //elf.push(64);
    //elf.push(0);
    // number of entries in the section header table
    elf.write_u16::<LittleEndian>(5).unwrap();
    //elf.push(5);
    //elf.push(0);
    // index of the section header table entry that contains the section names
    elf.write_u16::<LittleEndian>(2).unwrap();
    //elf.push(2);
    //elf.push(0);
}

fn write_program_header(elf: &mut Vec<u8>) {
    // make sure that we are at the proper position as specified in write_elf_header
    assert!(elf.len() == 64);

    // type of the segment: 1 is loadable segment
    elf.write_u32::<LittleEndian>(1).unwrap();
    // flags
    elf.write_u32::<LittleEndian>(5).unwrap();
    // segment's offset
    elf.write_u64::<LittleEndian>(0).unwrap();
    // segment's virtual address
    elf.write_u64::<LittleEndian>(0x400000).unwrap();
    // segment's physical address
    elf.write_u64::<LittleEndian>(0x400000).unwrap();
    // size of the segment in the file image
    elf.write_u64::<LittleEndian>(0x8C).unwrap();
    // size of the segment in memory
    elf.write_u64::<LittleEndian>(0x8C).unwrap();
    // alignment
    elf.write_u64::<LittleEndian>(0x200000).unwrap();
}

fn write_section_header(elf: &mut Vec<u8>, code_len: usize) {
    // make sure that we are at the proper position as specified in write_elf_header
    assert!(elf.len() == 64 + 56 + code_len);

    // offset to a string in the .shstrtab section that represents the name of this section
    elf.write_u32::<LittleEndian>(0).unwrap();
    // identifies the type of this header: 0 means entry is unused
    elf.write_u32::<LittleEndian>(0).unwrap();

    // identifies the attributes of the section
    elf.write_u64::<LittleEndian>(0).unwrap();
    // section's virtual address
    elf.write_u64::<LittleEndian>(0).unwrap();
    // section's offset
    elf.write_u64::<LittleEndian>(0).unwrap();
    // section's size
    elf.write_u64::<LittleEndian>(0).unwrap();
    // section index of an associated section
    elf.write_u32::<LittleEndian>(0).unwrap();
    // extra information about the section
    elf.write_u32::<LittleEndian>(0).unwrap();
    // section's required alignment
    elf.write_u64::<LittleEndian>(0).unwrap();
    // size of eache entry for sections that contain fixed-size entries, otherwise 0
    elf.write_u64::<LittleEndian>(0).unwrap();
}
*/
