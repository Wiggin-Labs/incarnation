use {Constant, Displacement, Error, Immediate, Instruction, Operand};

use string_interner::{INTERNER, Symbol};
use tokenizer::Token;

use std::num::{NonZeroI8, NonZeroI16, NonZeroI32};

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
}

fn get_symbol(token: Token, input: &str) -> Symbol {
    INTERNER.lock().unwrap().get_symbol(token.as_str(input).into())
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
                    Ok(t) if t.is_symbol() => {
                        let s = t.as_str(input);
                        if s.ends_with(':') {
                            instructions.push(Instruction::Label(get_symbol(t, input)));
                            continue;
                        } else {
                            return Err(Error::Opener);
                        }
                    }
                    Ok(t) if !t.openerp() => return Err(Error::Opener),
                    Err(_) => return Ok(instructions),
                    _ => break,
                }
            },
            Ok(t) if t.is_symbol() => {
                let s = t.as_str(input);
                if s.ends_with(':') {
                    instructions.push(Instruction::Label(get_symbol(t, input)));
                    continue;
                } else {
                    return Err(Error::Opener);
                }
            }
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
                    } else if mode == "label" {
                        parse_label(&mut parser, &mut operands, input)?;
                    } else if let Some(ty) = Constant::from_str(mode) {
                        parse_constant(ty, &mut parser, &mut operands, input)?;
                    } else {
                        return Err(Error::Mode);
                    }
                }
                Token::Integer(_) | Token::Float(_) => return Err(Error::NoType),
                // We assume this is a register, but valid register names differ based on
                // ISA, so we don't validate this until later.
                t @ Token::Symbol(_) => operands.push(Operand::Register(get_symbol(t, input))),
                _ => return Err(Error::Token),
            }
        }

        instructions.push(Instruction::new(get_symbol(opcode, input), operands));
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

    operands.push(Operand::Constant(Immediate::from(t.as_str(input), ty)?));
    Ok(())
}

fn parse_address(parser: &mut Parser, operands: &mut Vec<Operand>, input: &str)
    -> Result<(), Error>
{
    let operand = match parser.next()? {
        // This should be a displacement
        t if t.openerp() => parse_displacement(parser, input)?,
        t @ Token::Symbol(_) => Operand::Address(None, get_symbol(t, input)),
        _ => return Err(Error::Token),
    };
    if !parser.next()?.closerp() {
        return Err(Error::Token);
    }

    operands.push(operand);
    Ok(())
}

fn parse_label(parser: &mut Parser, operands: &mut Vec<Operand>, input: &str)
    -> Result<(), Error>
{
    let operand = match parser.next()? {
        t @ Token::Symbol(_) => Operand::Label(get_symbol(t, input)),
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

    Ok(Operand::Address(Some(displacement), get_symbol(reg, input)))
}
