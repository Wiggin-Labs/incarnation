use {Emitter, ModRM, Register, REX, SIB};

use asm_syntax::parser::Displacement;

use std::collections::HashMap;

pub struct Assembler {
    labels: HashMap<String, usize>,
    jumps: Vec<(String, usize)>,
    emitter: Emitter,
}

//include!(concat!(env!("OUT_DIR"), "/instructions.rs"));

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: HashMap::new(),
            jumps: Vec::new(),
            emitter: Emitter::new(),
        }
    }

    pub fn code(self) -> Vec<u8> {
        self.emitter.code()
    }

    pub fn append(&mut self, other: Vec<u8>) {
        self.emitter.append(other);
    }

    pub fn finish(mut self) -> Vec<u8> {
        for (label, i) in &self.jumps {
            let p = if let Some(p) = self.labels.get(label) {
                *p
            } else {
                // TODO
                panic!("Unknown label `{}`", label);
            };

            let offset = (p as isize - *i as isize) as i8 as u8;
            self.emitter.replace_byte_at_offset(*i, offset);
        }

        self.code()
    }

    pub fn label<S: Into<String>>(&mut self, label: S) {
        self.labels.insert(label.into(), self.emitter.len() - 1);
    }

    pub fn push_reg(&mut self, from: Register) {
        if from.rexp() {
            let rex = REX::new().b();
            self.emitter.emit_byte(*rex);
        }
        // push
        self.emitter.emit_byte(0x50 + from.value());
    }

    pub fn pop_reg(&mut self, to: Register) {
        if to.rexp() {
            let rex = REX::new().b();
            self.emitter.emit_byte(*rex);
        }
        // pop
        self.emitter.emit_byte(0x58 + to.value());
    }

    /*
    pub fn xor_reg_reg(&mut self, to: Register, from: Register) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().mod_(0b11)
            .reg_reg(from, &mut rex)
            .rm_reg(to, &mut rex);

        self.emitter.emit_byte(*rex);
        // xor
        self.emitter.emit_byte(0x31);
        self.emitter.emit_byte(*modrm);
    }

    pub fn jmp<S: Into<String>>(&mut self, label: S) {
        // jmp
        self.emitter.emit_byte(0xeb);
        self.jumps.push((label.into(), self.emitter.len()));
        self.emitter.emit_byte(0);
    }

    pub fn je<S: Into<String>>(&mut self, label: S) {
        // je
        self.emitter.emit_byte(0x74);
        self.jumps.push((label.into(), self.emitter.len()));
        self.emitter.emit_byte(0);
    }

    pub fn cmp(&mut self, to: Register, from: Register) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().mod_(0b11)
            .reg_reg(from, &mut rex)
            .rm_reg(to, &mut rex);

        self.emitter.emit_byte(*rex);
        self.emitter.emit_byte(0x39);
        self.emitter.emit_byte(*modrm);
    }
    */

    pub fn ret(&mut self) {
        // ret
        self.emitter.emit_byte(0xc3);
    }

    pub fn call_reg(&mut self, reg: Register) {
        let modrm = ModRM::new().mod_direct()
            // opcode extension
            .reg(2)
            .rm_reg(reg);

        // Don't think REX is needed here
        /*
        if reg.rexp() {
            let rex = REX::new().b();
            self.emitter.emit_byte(*rex);
        }
        */
        // call
        self.emitter.emit_byte(0xff);
        self.emitter.emit_byte(*modrm);
    }

    pub fn mov_reg_reg(&mut self, to: Register, from: Register) {
        let modrm = ModRM::new()
            .mod_direct()
            .reg_reg(from)
            .rm_reg(to);

        if to.rexp() || from.rexp() {
            let mut rex = REX::new().w();
            if to.rexp() {
                rex.set_b();
            }
            if from.rexp() {
                rex.set_r();
            }
            self.emitter.emit_byte(*rex);
        }

        // mov
        self.emitter.emit_byte(0x89);
        self.emitter.emit_byte(*modrm);
    }

    pub fn mov_reg_u32(&mut self, to: Register, imm: u32) {
        if to.b64p() {
            let mut rex = REX::new().w();
            if to.rexp() {
                rex.set_b();
            }
            self.emitter.emit_byte(*rex);
        }

        self.emitter.emit_byte(0xb8 + to.value());
        self.emitter.emit_u32(imm as u32);
    }

    pub fn mov_reg_i32(&mut self, to: Register, imm: i32) {
        self.mov_reg_u32(to, imm as u32);
    }

    pub fn mov_reg_u64(&mut self, to: Register, imm: u64) {
        let mut rex = REX::new().w();
        if to.rexp() {
            rex.set_b();
        }

        self.emitter.emit_byte(*rex);
        // movabs
        self.emitter.emit_byte(0xb8 + to.value());
        self.emitter.emit_u64(imm);
    }

    pub fn mov_reg_i64(&mut self, to: Register, imm: i64) {
        self.mov_reg_u64(to, imm as u64);
    }

    pub fn mov_addr_i32(&mut self, to: Register, displacement: Option<Displacement>, imm: i32) {
        let mut rex = REX::new().w();
        if to.rexp() {
            rex.set_b();
        }
        let modrm = ModRM::new().rm_addr(to);

        self.emitter.emit_byte(*rex);
        // movq
        self.emitter.emit_byte(0xc7);
        self.emitter.emit_byte(*modrm);
        if modrm.sibp() {
            let sib = SIB::new().base_reg(to);
            self.emitter.emit_byte(*sib);
        }
        if let Some(displacement) = displacement {
            match displacement {
                Displacement::Disp8(i) => {
                }
                Displacement::Disp16(i) => {
                }
                Displacement::Disp32(i) => {
                }
            }
        }
        self.emitter.emit_u32(imm as u32);
    }

    pub fn mov_addr_u8(&mut self, to: Register, imm: u8) {
        let modrm = ModRM::new().rm(to.value());

        // movb
        self.emitter.emit_byte(0xc6);
        self.emitter.emit_byte(*modrm);
        if modrm.sibp() {
            let sib = SIB::new().base_reg(to);
            self.emitter.emit_byte(*sib);
        }
        self.emitter.emit_byte(imm);
    }

    /*
    pub fn mov_reg_addr(&mut self, to: Register, addr: Register) {
        let mut rex = REX::new().w();
        // NOTE that 0x8b switches the order from 0x89 in mov_addr_reg
        let modrm = ModRM::new()
            .rm_reg(addr, &mut rex)
            .reg_addr(to, &mut rex);
        self.emitter.emit_byte(*rex);
        self.emitter.emit_byte(0x8b);
        self.emitter.emit_byte(*modrm);
    }

    pub fn mov_addr_reg(&mut self, addr: Register, from: Register) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new()
            .rm_addr(addr, &mut rex)
            .reg_reg(from, &mut rex);

        self.emitter.emit_byte(*rex);
        self.emitter.emit_byte(0x89);
        self.emitter.emit_byte(*modrm);
    }

    pub fn add_reg_reg(&mut self, to: Register, from: Register) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().mod_(0b11)
            .reg_reg(from, &mut rex)
            .rm_reg(to, &mut rex);

        self.emitter.emit_byte(*rex);
        // add
        self.emitter.emit_byte(0x01);
        self.emitter.emit_byte(*modrm);
    }

    pub fn add_reg_u8(&mut self, to: Register, imm: u8) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().mod_(0b11).rm_reg(to, &mut rex);

        self.emitter.emit_byte(*rex);
        // add
        self.emitter.emit_byte(0x83);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_byte(imm);
    }

    pub fn add_addr_u8(&mut self, addr: Register, imm: u8) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().rm_addr(addr, &mut rex);

        self.emitter.emit_byte(*rex);
        // add
        self.emitter.emit_byte(0x83);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_byte(imm);
    }
    */

    pub fn sub_reg_u64(&mut self, to: Register, imm: u64) {
        let mut rex = REX::new().w();
        if to.rexp() {
            rex.set_b();
        }
        self.emitter.emit_byte(*rex);

        let modrm = ModRM::new().mod_direct()
            // opcode extension
            .reg(5)
            .rm_reg(to);

        self.emitter.emit_byte(0x83);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_u64(imm);
    }

    pub fn sub_reg_u8(&mut self, to: Register, imm: u8) {
        if to.b64p() {
            let mut rex = REX::new().w();
            if to.rexp() {
                rex.set_b();
            }
            self.emitter.emit_byte(*rex);
        }

        let modrm = ModRM::new().mod_direct()
            // opcode extension
            .reg(5)
            .rm_reg(to);

        self.emitter.emit_byte(0x83);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_byte(imm);

        /*
        let mut rex = REX::new().w();
        let modrm = ModRM::new()
            .mod_(0b11)
            .rm_reg(addr, &mut rex)
            .reg(5);

        self.emitter.emit_byte(*rex);
        // sub
        self.emitter.emit_byte(0x83);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_byte(imm);
        */
    }

    /*
    pub fn sub_addr_u8(&mut self, addr: Register, imm: u8) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new()
            .rm_addr(addr, &mut rex)
            .reg(5);

        self.emitter.emit_byte(*rex);
        // sub
        self.emitter.emit_byte(0x83);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_byte(imm);
    }

    pub fn shl_reg_u8(&mut self, to: Register, imm: u8) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().mod_(0b11)
            .rm_reg(to, &mut rex)
            .reg(4);

        self.emitter.emit_byte(*rex);
        // shl
        self.emitter.emit_byte(0xc1);
        self.emitter.emit_byte(*modrm);
        self.emitter.emit_byte(imm);
    }

    pub fn inc_addr(&mut self, addr: Register) {
        let mut rex = REX::new().w();
        let modrm = ModRM::new().rm_addr(addr, &mut rex);
        self.emitter.emit_byte(*rex);
        // inc
        self.emitter.emit_byte(0xff);
        self.emitter.emit_byte(*modrm);
    }
    */

    pub fn syscall(&mut self) {
        self.emitter.emit_byte(0x0f);
        self.emitter.emit_byte(0x05);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        /*
        let code = vec![0x48, 0x31, 0xd2, // xor rdx, rdx
                        0x48, 0xc1, 0xe2, 0x03, // shl 3, rdx
                        0x48, 0x83, 0xc2, 0x10, // add 16, rdx
                        0x48, 0x01, 0xfa, // add rdi, rdx
                        0x48, 0xc7, 0x02, 0x05, 0, 0, 0, // movq 5, (rdx)
                        0x48, 0x89, 0xfa, // mov rdi, rdx
                        0x48, 0x83, 0xc2, 0x08, // add 8, rdx
                        0x48, 0xff, 0x02, // incq (rdx)
                        0x48, 0x31, 0xc0, // xor rax, rax
                        0x48, 0xc7, 0xc2, 1, 0, 0, 0, // movq 1, rdx
                        0x48, 0x83, 0xec, 8, // sub 8, rsp
                        0xc3]; // ret
        let mut asm = Assembler::new();
        asm.xor_reg_reg(Register::Rdx, Register::Rdx);
        asm.shl_reg_u8(Register::Rdx, 3);
        asm.add_reg_u8(Register::Rdx, 16);
        asm.add_reg_reg(Register::Rdx, Register::Rdi);
        asm.mov_addr_i32(Register::Rdx, 5);
        asm.mov_reg_reg(Register::Rdx, Register::Rdi);
        asm.add_reg_u8(Register::Rdx, 8);
        asm.inc_addr(Register::Rdx);
        asm.xor_reg_reg(Register::Rax, Register::Rax);
        asm.mov_reg_i32(Register::Rdx, 1);
        asm.sub_reg_u8(Register::Rsp, 8);
        asm.ret();
        assert_eq!(code, asm.finish());

        let code = vec![0xff, 0xd0, // call *rax
                        0x41, 0xff, 0xd0, // call *r8
                        0x48, 0xb8, 0x05, 0, 0, 0, 0, 0, 0, 0, // movabs 5, rax
        ];
        let mut asm = Assembler::new();
        asm.call(Register::Rax);
        asm.call(Register::R8);
        asm.mov_reg_i64(Register::Rax, 5);
        assert_eq!(code, asm.finish());

        let code = vec![0x48, 0x39, 0xfa, // cmp rdi, rdx
                        0x74, 0x09, // je ELSE
                        0x48, 0xc7, 0xc0, 5, 0, 0, 0, // mov 5, rax
                        0xeb, 0x07, // jmp DONE
                        // ELSE
                        0x48, 0xc7, 0xc0, 4, 0, 0, 0, // mov 4, rax
                        // DONE
                        0x48, 0xc7, 0xc0, 3, 0, 0, 0, // mov 3, rax
                        0xeb, 0xf7, // jmp DONE
        ];
        let mut asm = Assembler::new();
        asm.cmp(Register::Rdx, Register::Rdi);
        //asm.je(0x09);
        asm.je("ELSE");
        asm.mov_reg_i32(Register::Rax, 5);
        //asm.jmp(0x07);
        asm.jmp("DONE");
        asm.label("ELSE");
        asm.mov_reg_i32(Register::Rax, 4);
        asm.label("DONE");
        asm.mov_reg_i32(Register::Rax, 3);
        //asm.jmp(-9);
        asm.jmp("DONE");
        assert_eq!(code, asm.finish());

        let code = vec![0x48, 0x89, 0x07, // mov rax, (rdi)
                        0x48, 0x8b, 0x07, // mov (rdi), rax
                        0x48, 0x83, 0, 0x08, // add 8, (rax)
                        0x48, 0x83, 0x28, 0x08, // sub 8, (rax)
                        0xc6, 0x04, 0x24, 0x48, // movb 48, (rsp)
                        0x4d, 0x89, 0xc1, // mov r8, r9
        ];
        let mut asm = Assembler::new();
        asm.mov_addr_reg(Register::Rdi, Register::Rax);
        asm.mov_reg_addr(Register::Rax, Register::Rdi);
        asm.add_addr_u8(Register::Rax, 8);
        asm.sub_addr_u8(Register::Rax, 8);
        asm.mov_addr_u8(Register::Rsp, 0x48);
        asm.mov_reg_reg(Register::R9, Register::R8);
        assert_eq!(code, asm.finish());
        */

        let code = vec![0x4d, 0x89, 0xc1,
                        0x89, 0xd8,
                        //0x45, 0x89, 0xc1,
                        0x48, 0x83, 0xec, 8, // sub 8, rsp
                        0xc6, 0x04, 0x24, 0x48,
                        0x48, 0xc7, 0xc0, 1, 0, 0, 0,
        ];
        let mut asm = Assembler::new();
        asm.mov_reg_reg(Register::R9, Register::R8);
        asm.mov_reg_reg(Register::EAX, Register::EBX);
        //asm.mov_reg_reg(Register::R9D, Register::R8D);
        asm.sub_reg_u8(Register::RSP, 8);
        asm.mov_addr_u8(Register::RSP, 72);
        asm.mov_reg_i32(Register::RAX, 1);
        assert_eq!(code, asm.finish());
    }
}
