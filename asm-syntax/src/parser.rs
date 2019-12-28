use tokenizer::Token;

use std::fmt::{self, Display, Formatter};
use std::num::{NonZeroI8, NonZeroI16, NonZeroI32};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    EndOfInput,
    Closer,
    Opener,
    Mode,
    Token,
    NoType,
    ConstantType,
    InvalidDisplacement
}

impl From<std::num::ParseIntError> for Error {
    fn from(_err: std::num::ParseIntError) -> Error {
        Error::InvalidDisplacement
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::EndOfInput => "Unexpected end of input",
            Error::Closer => "Unexpected closer",
            Error::Opener => "Unexpected opener",
            Error::Mode => "Invalid mode",
            Error::Token => "Unexpected token",
            Error::NoType => "Expected a type specifier",
            Error::ConstantType => "Given value does not match expected type",
            Error::InvalidDisplacement => "Invalid value given for displacement",
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Token,
    pub operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(opcode: Token, operands: Vec<Operand>) -> Self {
        Instruction {
            opcode,
            operands,
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, is_enum_variant)]
pub enum Operand {
    Register(Token),
    Address(Option<Displacement>, Token),
    Constant(Constant, Token),
}

impl Operand {
    pub fn unwrap_register(self) -> Token {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Immediate {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Immediate {
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

struct Parser<'a> {
    position: usize,
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser {
            position: 0,
            tokens: tokens,
        }
    }

    fn next(&mut self) -> Result<Token, Error> {
        if self.position < self.tokens.len() {
            self.position += 1;
            Ok(self.tokens[self.position - 1])
        } else {
            Err(Error::EndOfInput)
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position])
        } else {
            None
        }
    }
}

pub fn parse(tokens: &[Token], input: &str, inlinep: bool) -> Result<Vec<Instruction>, Error> {
    let mut parser = Parser::new(tokens);
    let mut instructions = Vec::new();

    loop {
        match parser.next() {
            Ok(t) if t.closerp() => {
                if inlinep {
                    return Ok(instructions);
                } else {
                    return Err(Error::Closer);
                }
            }
            // Skip over comments
            Ok(t) if t.commentp() => loop {
                match parser.next() {
                    Ok(t) if t.closerp() => {
                        if inlinep {
                            return Ok(instructions);
                        } else {
                            return Err(Error::Closer);
                        }
                    }
                    Ok(t) if t.commentp() => (),
                    Ok(t) if !t.openerp() => return Err(Error::Opener),
                    Err(_) => return Ok(instructions),
                    _ => break,
                }
            },
            Ok(t) if !t.openerp() => return Err(Error::Opener),
            // If this is inline assembly we expect it to end with an extra closer, which isn't
            // there if we've reached end of input.
            Err(_) => if inlinep {
                return Err(Error::EndOfInput);
            } else {
                return Ok(instructions);
            },
            _ => (),
        }

        let opcode = match parser.next()? {
            t @ Token::Symbol(_) => t,
            // Skip over comments
            t if t.commentp() => loop {
                match parser.next()? {
                    t @ Token::Symbol(_) => break t,
                    // continue skipping over comments
                    t if t.commentp() => (),
                    _ => return Err(Error::Token),
                }
            },
            _ => return Err(Error::Token),
        };

        let mut operands = Vec::new();
        // Parse opcode arguments
        loop {
            match parser.next()? {
                // Done parsing operands
                t if t.closerp() => break,
                t if t.openerp() => {
                    let t = parser.next()?;
                    let mode = if t.is_symbol() {
                        t.as_str(input)
                    } else {
                        return Err(Error::Token);
                    };

                    if mode == "address" {
                        parse_address(&mut parser, &mut operands, input)?;
                    } else if let Some(ty) = Constant::from_str(mode) {
                        parse_constant(ty, &mut parser, &mut operands, input)?;
                    } else {
                        return Err(Error::Mode);
                    }
                }
                Token::Integer(_) | Token::Float(_) => return Err(Error::NoType),
                // We assume this is a register, but valid register names differ based on
                // ISA, so we don't validate this until later.
                t @ Token::Symbol(_) => operands.push(Operand::Register(t)),
                _ => return Err(Error::Token),
            }
        }

        instructions.push(Instruction::new(opcode, operands));
    }
}

fn parse_constant(ty: Constant, parser: &mut Parser, operands: &mut Vec<Operand>, input: &str)
    -> Result<(), Error>
{
    let t = parser.next()?;
    if !t.is_integer() && !t.is_float() {
        return Err(Error::Token);
    }

    // Validate proper types
    if (ty.unsignedp() && (t.is_float() || t.as_str(input).starts_with('-'))) ||
        (ty.signedp() && t.is_float()) ||
        (ty.floatp() && t.is_integer())
    {
        return Err(Error::ConstantType);
    }

    assert!(parser.next()?.closerp());

    operands.push(Operand::Constant(ty, t));
    Ok(())
}

fn parse_address(parser: &mut Parser, operands: &mut Vec<Operand>, input: &str)
    -> Result<(), Error>
{
    let operand = match parser.next()? {
        // This should be a displacement
        t if t.openerp() => parse_displacement(parser, input)?,
        t @ Token::Symbol(_) => Operand::Address(None, t),
        _ => return Err(Error::Token),
    };
    if !parser.next()?.closerp() {
        return Err(Error::Token);
    }

    operands.push(operand);
    Ok(())
}

fn parse_displacement(parser: &mut Parser, input: &str) -> Result<Operand, Error> {
    let t = parser.next()?;
    let op = if t.is_symbol() {
        t.as_str(input)
    } else {
        return Err(Error::Token);
    };

    let (operand_one, ty1) = {
        match parser.next()? {
            t if t.is_symbol() => (t, None),
            t if t.openerp() => {
                let ty = parser.next()?;
                assert!(ty.is_symbol());
                let value = parser.next()?;
                assert!(ty.is_integer());
                assert!(parser.next()?.closerp());
                (value, Some(ty))
            }
            _ => return Err(Error::Token),
        }
    };

    let (operand_two, ty2) = {
        match parser.next()? {
            t if t.is_symbol() && !operand_one.is_symbol() => (t, None),
            t if t.openerp() && operand_one.is_symbol() => {
                let ty = parser.next()?;
                assert!(ty.is_symbol());
                let value = parser.next()?;
                assert!(ty.is_integer());
                assert!(parser.next()?.closerp());
                (value, Some(ty))
            }
            _ => return Err(Error::Token),
        }
    };

    // eat displacement closer
    if !parser.next()?.closerp() {
        return Err(Error::Token);
    }

    let (mut displacement, reg) = if operand_one.is_integer() {
        let d = operand_one.as_str(input);
        let ty = if let Some(c) = Constant::from_str(ty1.unwrap().as_str(input)) {
            c
        } else {
            return Err(Error::Mode);
        };

        let d = match ty {
            Constant::I8 => Displacement::Disp8(d.parse::<NonZeroI8>()?),
            Constant::I16 => Displacement::Disp16(d.parse::<NonZeroI16>()?),
            Constant::I32 => Displacement::Disp32(d.parse::<NonZeroI32>()?),
            _ => return Err(Error::Mode),
        };

        (d, operand_two)
    } else {
        let d = operand_two.as_str(input);
        let ty = if let Some(c) = Constant::from_str(ty2.unwrap().as_str(input)) {
            c
        } else {
            return Err(Error::Mode);
        };

        let d = match ty {
            Constant::I8 => Displacement::Disp8(d.parse::<NonZeroI8>()?),
            Constant::I16 => Displacement::Disp16(d.parse::<NonZeroI16>()?),
            Constant::I32 => Displacement::Disp32(d.parse::<NonZeroI32>()?),
            _ => return Err(Error::Mode),
        };

        (d, operand_one)
    };

    match op {
        "+" => (),
        "-" => displacement.negate(),
        // TODO
        _ => return Err(Error::Token),
    };

    Ok(Operand::Address(Some(displacement), reg))
}
