#[macro_use]
extern crate derive_is_enum_variant;
extern crate string_interner;
extern crate tokenizer;

mod error;
pub mod parser;

pub use error::Error;

use string_interner::Symbol;

use std::num::{NonZeroI8, NonZeroI16, NonZeroI32};

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Label(Symbol),
    Constant(Symbol, Immediate),
    Operation(Operation),
}

impl Instruction {
    pub fn new(opcode: Symbol, operands: Vec<Operand>) -> Self {
        Instruction::Operation(Operation {
            opcode,
            operands,
        })
    }

    pub fn unwrap_label(self) -> Symbol {
        match self {
            Instruction::Label(s) => s,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub opcode: Symbol,
    pub operands: Vec<Operand>,
}

// NOTE: on AMD64 displacements are limited to signed 32 bits, may be different on other
// architectures. I don't know of any other ISA's that support displacement though.
// We use a NonZero because storing a displacement of 0 seems silly and this means that
// Option<Displacement> doesn't require an extra byte. I think supporting displacement on RISC ISAs
// is a bad idea, but if I change my mind I may just change it to i32 and remove the option.
//type Displacement = NonZeroI32;
#[derive(Debug, Copy, Clone, PartialEq, is_enum_variant)]
pub enum Displacement {
    Disp8(NonZeroI8),
    Disp16(NonZeroI16),
    Disp32(NonZeroI32),
}

impl Displacement {
    pub fn negate(&mut self) {
        use self::Displacement::*;
        match self {
            Disp8(i) => *self = Disp8(NonZeroI8::new(-(i.get())).unwrap()),
            Disp16(i) => *self = Disp16(NonZeroI16::new(-(i.get())).unwrap()),
            Disp32(i) => *self = Disp32(NonZeroI32::new(-(i.get())).unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, is_enum_variant)]
pub enum Operand {
    Register(Symbol),
    Address(Option<Displacement>, Symbol),
    Constant(Immediate),
    Label(Symbol),
}

impl Operand {
    pub fn unwrap_register(self) -> Symbol {
        match self {
            Operand::Register(t) | Operand::Address(_, t) => t,
            _ => unreachable!(),
        }
    }

    pub fn unwrap_disp(self) -> Option<Displacement> {
        match self {
            Operand::Address(d, _) => d,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Constant {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Immediate {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Bytes(Vec<u8>),
}

impl Immediate {
    pub fn from(t: &str, ty: Constant) -> Result<Self, std::num::ParseIntError> {
        Ok(match ty {
            Constant::U8 => Immediate::U8(t.parse::<u8>()?),
            Constant::U16 => Immediate::U16(t.parse::<u16>()?),
            Constant::U32 => Immediate::U32(t.parse::<u32>()?),
            Constant::U64 => Immediate::U64(t.parse::<u64>()?),
            Constant::I8 => Immediate::I8(t.parse::<i8>()?),
            Constant::I16 => Immediate::I16(t.parse::<i16>()?),
            Constant::I32 => Immediate::I32(t.parse::<i32>()?),
            Constant::I64 => Immediate::I64(t.parse::<i64>()?),
        })
    }

    pub fn to_le_bytes(self) -> Vec<u8> {
        match self {
            Immediate::Bytes(v) => v,
            Immediate::U8(i) => i.to_le_bytes().to_vec(),
            Immediate::U16(i) => i.to_le_bytes().to_vec(),
            Immediate::U32(i) => i.to_le_bytes().to_vec(),
            Immediate::U64(i) => i.to_le_bytes().to_vec(),
            Immediate::I8(i) => i.to_le_bytes().to_vec(),
            Immediate::I16(i) => i.to_le_bytes().to_vec(),
            Immediate::I32(i) => i.to_le_bytes().to_vec(),
            Immediate::I64(i) => i.to_le_bytes().to_vec(),
        }
    }

    pub fn b64p(&self) -> bool {
        use self::Immediate::*;
        match self {
            U64(_) | I64(_) => true,
            _ => false,
        }
    }

    pub fn b32p(&self) -> bool {
        use self::Immediate::*;
        match self {
            U32(_) | I32(_) => true,
            _ => false,
        }
    }

    pub fn b16p(&self) -> bool {
        use self::Immediate::*;
        match self {
            U16(_) | I16(_) => true,
            _ => false,
        }
    }

    pub fn b8p(&self) -> bool {
        use self::Immediate::*;
        match self {
            U8(_) | I8(_) => true,
            _ => false,
        }
    }
}

impl Constant {
    pub fn unsignedp(self) -> bool {
        use self::Constant::*;
        match self {
            U8 | U16 | U32 | U64 => true,
            _ => false,
        }
    }

    pub fn signedp(self) -> bool {
        use self::Constant::*;
        match self {
            I8 | I16 | I32 | I64 => true,
            _ => false,
        }
    }

    pub fn floatp(self) -> bool {
        // TODO
        false
    }

    pub fn from_str(input: &str) -> Option<Self> {
        Some(match input {
            "u8" | "U8" => Constant::U8,
            "u16" | "U16" => Constant::U16,
            "u32" | "U32" => Constant::U32,
            "u64" | "U64" => Constant::U64,
            "i8" | "I8" => Constant::I8,
            "i16" | "I16" => Constant::I16,
            "i32" | "I32" => Constant::I32,
            "i64" | "I64" => Constant::I64,
            _ => return None,
        })
    }
}
