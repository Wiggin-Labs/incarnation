use asm_syntax::Immediate;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
#[repr(u8)]
pub enum Register {
    // 8-bit GP, 16-bit GP, 32-bit GP, 64-bit GP, 64-bit MMX, 128-bit XMM
    AL, AX, EAX, RAX, MMX0, XMM0,
    BL, BX, EBX, RBX, MMX1, XMM1,
    CL, CX, ECX, RCX, MMX2, XMM2,
    DL, DX, EDX, RDX, MMX3, XMM3,
    CH, BP, EBP, RBP, MMX4, XMM4,
    AH, SP, ESP, RSP, MMX5, XMM5,
    DH, SI, ESI, RSI, MMX6, XMM6,
    BH, DI, EDI, RDI, MMX7, XMM7,
    R8L, R8W, R8D, R8,
    R9L, R9W, R9D, R9,
    R10L, R10W, R10D, R10,
    R11L, R11W, R11D, R11,
    R12L, R12W, R12D, R12,
    R13L, R13W, R13D, R13,
    R14L, R14W, R14D, R14,
    R15L, R15W, R15D, R15,
}

impl Register {
    pub fn from_str(input: &str) -> Option<Self> {
        Some(match input {
            "al" | "AL" => Register::AL,
            "bl" | "BL" => Register::BL,
            "cl" | "CL" => Register::CL,
            "dl" | "DL" => Register::DL,
            "ah" | "AH" => Register::AH,
            "bh" | "BH" => Register::BH,
            "ch" | "CH" => Register::CH,
            "dh" | "DH" => Register::DH,
            "r8l" | "R8L" => Register::R8L,
            "r9l" | "R9L" => Register::R9L,
            "r10l" | "R10L" => Register::R10L,
            "r11l" | "R11L" => Register::R11L,
            "r12l" | "R12L" => Register::R12L,
            "r13l" | "R13L" => Register::R13L,
            "r14l" | "R14L" => Register::R14L,
            "r15l" | "R15L" => Register::R15L,
            "ax" | "AX" => Register::AX,
            "bx" | "BX" => Register::BX,
            "cx" | "CX" => Register::CX,
            "dx" | "DX" => Register::DX,
            "bp" | "BP" => Register::BP,
            "sp" | "SP" => Register::SP,
            "si" | "SI" => Register::SI,
            "di" | "DI" => Register::DI,
            "r8w" | "R8W" => Register::R8W,
            "r9w" | "R9W" => Register::R9W,
            "r10w" | "R10W" => Register::R10W,
            "r11w" | "R11W" => Register::R11W,
            "r12w" | "R12W" => Register::R12W,
            "r13w" | "R13W" => Register::R13W,
            "r14w" | "R14W" => Register::R14W,
            "r15w" | "R15W" => Register::R15W,
            "eax" | "EAX" => Register::EAX,
            "ebx" | "EBX" => Register::EBX,
            "ecx" | "ECX" => Register::ECX,
            "edx" | "EDX" => Register::EDX,
            "ebp" | "EBP" => Register::EBP,
            "esp" | "ESP" => Register::ESP,
            "esi" | "ESI" => Register::ESI,
            "edi" | "EDI" => Register::EDI,
            "r8d" | "R8D" => Register::R8D,
            "r9d" | "R9D" => Register::R9D,
            "r10d" | "R10D" => Register::R10D,
            "r11d" | "R11D" => Register::R11D,
            "r12d" | "R12D" => Register::R12D,
            "r13d" | "R13D" => Register::R13D,
            "r14d" | "R14D" => Register::R14D,
            "r15d" | "R15D" => Register::R15D,
            "rax" | "RAX" => Register::RAX,
            "rbx" | "RBX" => Register::RBX,
            "rcx" | "RCX" => Register::RCX,
            "rdx" | "RDX" => Register::RDX,
            "rbp" | "RBP" => Register::RBP,
            "rsp" | "RSP" => Register::RSP,
            "rsi" | "RSI" => Register::RSI,
            "rdi" | "RDI" => Register::RDI,
            "r8" | "R8" => Register::R8,
            "r9" | "R9" => Register::R9,
            "r10" | "R10" => Register::R10,
            "r11" | "R11" => Register::R11,
            "r12" | "R12" => Register::R12,
            "r13" | "R13" => Register::R13,
            "r14" | "R14" => Register::R14,
            "r15" | "R15" => Register::R15,
            "mmx0" | "MMX0" => Register::MMX0,
            "mmx1" | "MMX1" => Register::MMX1,
            "mmx2" | "MMX2" => Register::MMX2,
            "mmx3" | "MMX3" => Register::MMX3,
            "mmx4" | "MMX4" => Register::MMX4,
            "mmx5" | "MMX5" => Register::MMX5,
            "mmx6" | "MMX6" => Register::MMX6,
            "mmx7" | "MMX7" => Register::MMX7,
            "xmm0" | "XMM0" => Register::XMM0,
            "xmm1" | "XMM1" => Register::XMM1,
            "xmm2" | "XMM2" => Register::XMM2,
            "xmm3" | "XMM3" => Register::XMM3,
            "xmm4" | "XMM4" => Register::XMM4,
            "xmm5" | "XMM5" => Register::XMM5,
            "xmm6" | "XMM6" => Register::XMM6,
            "xmm7" | "XMM7" => Register::XMM7,
            _ => return None,
        })
    }

    pub fn value(self) -> u8 {
        use Register::*;
        match self {
            AL | AX | EAX | RAX |
            R8L | R8W | R8D | R8 |
                MMX0 | XMM0 => 0b000,
            CL | CX | ECX | RCX |
            R9L | R9W | R9D | R9 |
                MMX1 | XMM1 => 0b001,
            DL | DX | EDX | RDX |
            R10L | R10W | R10D | R10 |
                MMX2 | XMM2 => 0b010,
            BL | BX | EBX | RBX |
            R11L | R11W | R11D | R11 |
                MMX3 | XMM3 => 0b011,
            AH | SP | ESP | RSP |
            R12L | R12W | R12D | R12 |
                MMX4 | XMM4 => 0b100,
            CH | BP | EBP | RBP |
            R13L | R13W | R13D | R13 |
                MMX5 | XMM5 => 0b101,
            DH | SI | ESI | RSI |
            R14L | R14W | R14D | R14 |
                MMX6 | XMM6 => 0b110,
            BH | DI | EDI | RDI |
            R15L | R15W | R15D | R15 |
                MMX7 | XMM7 => 0b111,
        }
    }

    pub fn rexp(&self) -> bool {
        use Register::*;
        match self {
            R8L | R8W | R8D | R8 |
            R9L | R9W | R9D | R9 |
            R10L | R10W | R10D | R10 |
            R11L | R11W | R11D | R11 |
            R12L | R12W | R12D | R12 |
            R13L | R13W | R13D | R13 |
            R14L | R14W | R14D | R14 |
            R15L | R15W | R15D | R15 => true,
            _ => false,
        }
    }

    pub fn b64p(&self) -> bool {
        use Register::*;
        match self {
            RAX | RCX | RDX | RBX | RSP | RBP | RSI | RDI |
                R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 => true,
            _ => false,
        }
    }

    pub fn b32p(&self) -> bool {
        use Register::*;
        match self {
            EAX | ECX | EDX | EBX | ESP | EBP | ESI | EDI |
                R8D | R9D | R10D | R11D | R12D | R13D | R14D | R15D => true,
            _ => false,
        }
    }

    pub fn b16p(&self) -> bool {
        use Register::*;
        match self {
            AX | CX | DX | BX | SP | BP | SI | DI |
                R8W | R9W | R10W | R11W | R12W | R13W | R14W | R15W => true,
            _ => false,
        }
    }

    pub fn b8p(&self) -> bool {
        use Register::*;
        match self {
            AL | CL | DL | BL | AH | CH | DH | BH |
                R8L | R9L | R10L | R11L | R12L | R13L | R14L | R15L => true,
            _ => false,
        }
    }

    pub fn matches_imm(&self, imm: Immediate) -> bool {
        if self.b8p() {
            imm.b8p()
        } else if self.b16p() {
            // TODO: not sure if these should include smaller values or not
            imm.b8p() || imm.b16p()
        } else if self.b32p() {
            imm.b8p() || imm.b16p() || imm.b32p()
        } else if self.b64p() {
            imm.b8p() || imm.b16p() || imm.b32p() || imm.b64p()
        } else {
            unreachable!();
        }
    }

    pub fn matches_reg(&self, reg: Register) -> bool {
        if self.b8p() {
            reg.b8p()
        } else if self.b16p() {
            reg.b16p()
        } else if self.b32p() {
            reg.b32p()
        } else if self.b64p() {
            reg.b64p()
        } else {
            unreachable!();
        }
    }
}
