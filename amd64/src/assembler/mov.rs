use super::Assembler;
use {ASO, ModRM, OSO, Register, REX, SIB};

use asm_syntax::parser::{Displacement, Immediate};

impl Assembler {
    pub fn mov_reg_imm(&mut self, to: Register, imm: Immediate) {
        assert!(to.matches_imm(imm));
        if to.b16p() {
            self.emitter.emit_byte(*OSO::new());
        }

        if to.b64p() || to.rexp() {
            let mut rex = REX::new();
            if to.b64p() {
                rex.set_w();
            }
            if to.rexp() {
                rex.set_b();
            }

            self.emitter.emit_byte(*rex);
        }

        let opcode = if to.b8p() {
            0xb0 + to.value()
        } else if imm.b64p() || !to.b64p() {
            0xb8 + to.value()
        } else {
            0xc7
        };
        self.emitter.emit_byte(opcode);

        if to.b64p() && !imm.b64p() {
            let modrm = ModRM::new().mod_direct().rm_reg(to);
            self.emitter.emit_byte(*modrm);
        }
        self.emitter.emit_imm(imm);
    }

    pub fn mov_reg_reg(&mut self, to: Register, from: Register) {
        assert!(to.matches_reg(from));

        if to.b16p() {
            self.emitter.emit_byte(*OSO::new());
        }

        if to.b64p() || to.rexp() || from.rexp() {
            let mut rex = REX::new();
            if to.b64p() {
                rex.set_w();
            }
            if to.rexp() {
                rex.set_b();
            }
            if from.rexp() {
                rex.set_r();
            }
            self.emitter.emit_byte(*rex);
        }

        // mov
        let opcode = if to.b8p() {
            0x88
        } else {
            0x89
        };

        self.emitter.emit_byte(opcode);

        let modrm = ModRM::new()
            .mod_direct()
            .reg_reg(from)
            .rm_reg(to);

        self.emitter.emit_byte(*modrm);
    }

    pub fn mov_addr_imm(&mut self, to: Register, displacement: Option<Displacement>, imm: Immediate) {
        assert!(to.matches_imm(imm));
        if to.b16p() {
            self.emitter.emit_byte(*OSO::new());
        }

        if to.b64p() || to.rexp() {
            let mut rex = REX::new();
            if to.b64p() {
                rex.set_w();
            }
            if to.rexp() {
                rex.set_b();
            }

            self.emitter.emit_byte(*rex);
        }

        let opcode = if imm.b8p() {
            0xc6
        } else {
            0xc7
        };

        self.emitter.emit_byte(opcode);

        let mut modrm = ModRM::new()
            .reg_addr(to);
        if displacement.is_some() {
            modrm.set_mod_indirect();
        }
        self.emitter.emit_byte(*modrm);

        // TODO
        if modrm.sibp() {
            let sib = SIB::new().base_reg(to);
            self.emitter.emit_byte(*sib);
        }

        if displacement.is_some() {
            self.emitter.emit_displacement(displacement.unwrap());
        }
        self.emitter.emit_imm(imm);
    }

    pub fn mov_reg_addr(&mut self, to: Register, addr: Register, displacement: Option<Displacement>) {
        assert!(addr.b64p() || addr.b32p());

        if addr.b32p() {
            self.emitter.emit_byte(*ASO::new());
        }

        if to.b64p() || to.rexp() || addr.rexp() {
            let mut rex = REX::new();
            if to.b64p() {
                rex.set_w();
            }
            if to.rexp() {
                rex.set_b();
            }
            if addr.rexp() {
                rex.set_r();
            }
            self.emitter.emit_byte(*rex);
        }

        // NOTE that 0x8b switches the order from 0x89 in mov_addr_reg
        self.emitter.emit_byte(0x8b);

        let mut modrm = ModRM::new()
            .reg_addr(to)
            .rm_reg(addr);
        if displacement.is_some() {
            modrm.set_mod_indirect();
        }
        self.emitter.emit_byte(*modrm);

        if displacement.is_some() {
            self.emitter.emit_displacement(displacement.unwrap());
        }
    }

    pub fn mov_addr_reg(&mut self, addr: Register, from: Register, displacement: Option<Displacement>) {
        assert!(addr.b64p() || addr.b32p());

        if addr.b32p() {
            self.emitter.emit_byte(*ASO::new());
        }

        if from.b64p() || from.rexp() || addr.rexp() {
            let mut rex = REX::new();
            if from.b64p() {
                rex.set_w();
            }
            if from.rexp() {
                rex.set_b();
            }
            if addr.rexp() {
                rex.set_r();
            }
            self.emitter.emit_byte(*rex);
        }

        self.emitter.emit_byte(0x89);

        let mut modrm = ModRM::new()
            .reg_reg(from)
            .rm_addr(addr);
        if displacement.is_some() {
            modrm.set_mod_indirect();
        }
        self.emitter.emit_byte(*modrm);

        if displacement.is_some() {
            self.emitter.emit_displacement(displacement.unwrap());
        }
    }
}
