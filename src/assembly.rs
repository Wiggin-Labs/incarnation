use parser::Token;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    EndOfInput,
    Closer,
    Opener,
    Mode,
    Token,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::EndOfInput => "Unexpected end of input",
            Error::Closer => "Unexpected closer",
            Error::Opener => "Unexpected opener",
            Error::Mode => "Invalid mode",
            Error::Token => "Unexpected token",
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    Register(Token),
    Address(Token),
    Constant(Token),
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

    fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
            Some(self.tokens[self.position - 1])
        } else {
            None
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
            Some(t) if t.closerp() => {
                if inlinep {
                    return Ok(instructions);
                } else {
                    return Err(Error::Closer);
                }
            }
            Some(t) if !t.openerp() => return Err(Error::Opener),
            // If this is inline assembly we expect it to end with an extra closer, which isn't
            // there if we've reached end of input.
            None => if inlinep {
                return Err(Error::EndOfInput);
            } else {
                return Ok(instructions);
            },
            _ => (),
        }

        let opcode = if let Some(t) = parser.next() {
            match t {
                Token::Symbol(_) => t,
                // Skip over comments
                _ if t.commentp() => loop {
                    if let Some(t) = parser.next() {
                        match t {
                            Token::Symbol(_) => break t,
                            // continue skipping over comments
                            _ if t.commentp() => (),
                            _ => return Err(Error::Token),
                        }
                    } else {
                        return Err(Error::EndOfInput);
                    }
                },
                _ => return Err(Error::Token),
            }
        } else {
            return Err(Error::EndOfInput);
        };

        let mut operands = Vec::new();
        // Parse opcode arguments
        loop {
            if let Some(t) = parser.next() {
                match t {
                    // Done parsing operands
                    _ if t.closerp() => break,
                    _ if t.openerp() => {
                        let mode = if let Some(t) = parser.next() {
                            if let Token::Symbol(i) = t {
                                Token::Symbol(i).as_str(input)
                            } else {
                                return Err(Error::Token);
                            }
                        } else {
                            return Err(Error::EndOfInput);
                        };

                        let operand = if let Some(t) = parser.next() {
                            match t {
                                // We assume this is a register, but valid register names differ based on
                                // ISA, so we don't validate this until later.
                                Token::Symbol(_) => t,
                                _ => return Err(Error::Token),
                            }
                        } else {
                            return Err(Error::EndOfInput);
                        };

                        if let Some(t) = parser.next() {
                            // Special modes should only have one argument
                            if !t.closerp() {
                                return Err(Error::Token);
                            }
                        } else {
                            return Err(Error::EndOfInput);
                        }

                        if mode == "address" {
                            operands.push(Operand::Address(operand));
                        } else {
                            return Err(Error::Mode);
                        }
                    }
                    Token::Integer(_) => operands.push(Operand::Constant(t)),
                    Token::Float(_) => operands.push(Operand::Constant(t)),
                    // We assume this is a register, but valid register names differ based on
                    // ISA, so we don't validate this until later.
                    Token::Symbol(_) => operands.push(Operand::Register(t)),
                    _ => return Err(Error::Token),

                }
            } else {
                return Err(Error::EndOfInput);
            }
        }

        instructions.push(Instruction::new(opcode, operands));
    }
}
