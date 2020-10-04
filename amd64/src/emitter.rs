use asm_syntax::{Displacement, Immediate};
use byteorder::{LittleEndian, WriteBytesExt};

pub struct Emitter(Vec<u8>);

impl Emitter {
    pub fn new() -> Self {
        Emitter(vec![])
    }

    pub fn append(&mut self, mut other: Vec<u8>) {
        self.0.append(&mut other);
    }

    pub fn emit_imm(&mut self, imm: Immediate) {
        use asm_syntax::Immediate::*;
        match imm {
            U8(i) => self.emit_byte(i),
            I8(i) => self.emit_byte(i as u8),
            U16(i) => self.emit_u16(i),
            I16(i) => self.emit_u16(i as u16),
            U32(i) => self.emit_u32(i),
            I32(i) => self.emit_u32(i as u32),
            U64(i) => self.emit_u64(i),
            I64(i) => self.emit_u64(i as u64),
        }
    }

    pub fn emit_displacement(&mut self, d: Displacement) {
        use asm_syntax::Displacement::*;
        match d {
            Disp8(d) => self.emit_byte(d.get() as u8),
            Disp16(d) => self.emit_u16(d.get() as u16),
            Disp32(d) => self.emit_u32(d.get() as u32),
        }
    }

    pub fn emit_byte(&mut self, b: u8) {
        self.0.push(b);
    }

    /*
    pub fn emit_bytes(&mut self, b: &[u8]) {
        self.0.extend_from_slice(b);
    }
    */

    pub fn emit_u16(&mut self, b: u16) {
        self.0.write_u16::<LittleEndian>(b).unwrap();
    }

    pub fn emit_u32(&mut self, b: u32) {
        self.0.write_u32::<LittleEndian>(b).unwrap();
    }

    pub fn emit_u64(&mut self, b: u64) {
        self.0.write_u64::<LittleEndian>(b).unwrap();
    }

    pub fn replace_byte_at_offset(&mut self, offset: usize, b: u8) {
        self.0[offset] = b;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn code(self) -> Vec<u8> {
        self.0
    }
}
