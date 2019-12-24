use byteorder::{LittleEndian, WriteBytesExt};

pub struct Emitter(Vec<u8>);

impl Emitter {
    pub fn new() -> Self {
        Emitter(vec![])
    }

    pub fn append(&mut self, mut other: Vec<u8>) {
        self.0.append(&mut other);
    }

    pub fn emit_byte(&mut self, b: u8) {
        self.0.push(b);
    }

    /*
    pub fn emit_bytes(&mut self, b: &[u8]) {
        self.0.extend_from_slice(b);
    }
    */

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
