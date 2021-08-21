extern crate byteorder;
extern crate elf;
extern crate tokenizer;

#[macro_use]
mod macros;

mod assembler;
mod emitter;
mod register;

use emitter::Emitter;

use tokenizer::Token;

use std::env;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

pub use assembler::Assembler;
pub use register::Register;

fn main() {
    let input_file = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&input_file).unwrap();
    let tokens = tokenizer::Tokenizer::tokenize(&input).unwrap();
    let (program, data, rewrites) = Asm::assemble(tokens, &input);
    let e = elf::Elf::new(elf::ISA::Riscv, program, data, rewrites);
    let mut output_file = input_file.split(".").next().unwrap();
    if output_file == input_file {
        output_file = "bin.elf";
    }
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o777)
        .open(output_file)
        .unwrap();
    f.write_all(&e.to_vec()).unwrap();

}

struct Asm<'a> {
    asm: Assembler,
    data: Vec<Vec<u8>>,
    rewrites: HashMap<usize, usize>,
    constants: HashMap<&'a str, i32>,
    globals: HashMap<&'a str, usize>,
}

fn unwrap_register<'a, I: Iterator<Item = &'a Token>>(tokens: &mut I, input: &str) -> Register {
    if let Some(Token::Symbol(i)) = tokens.next() {
        Register::from_str(Token::Symbol(*i).as_str(input)).unwrap()
    } else {
        unreachable!();
    }
}

macro_rules! r {
    ($op:ident, $asm:expr, $tokens:ident, $input:ident) => {
        {
            let rd = unwrap_register($tokens, $input);
            let rs1 = unwrap_register($tokens, $input);
            let rs2 = unwrap_register($tokens, $input);
            assert!($tokens.next().unwrap().closerp());
            $asm.$op(rd, rs1, rs2);
        }
    };
}

macro_rules! i {
    ($op:ident, $self:ident, $tokens:ident, $input:ident) => {
        {
            let rd = unwrap_register($tokens, $input);
            let rs1 = unwrap_register($tokens, $input);
            let imm = $self.read_imm($tokens, $input);
            assert!($tokens.next().unwrap().closerp());
            $self.asm.$op(rd, rs1, imm);
        }
    };
}

fn offset<'a, I: Iterator<Item = &'a Token>>(tokens: &mut I, input: &str) -> (Register, i32) {
    match tokens.next().unwrap() {
        s @ Token::Symbol(_) => (Register::from_str(s.as_str(input)).unwrap(), 0),
        Token::LParen(_) => {
            let negate = match tokens.next().unwrap() {
                s @ Token::Symbol(_) => match s.as_str(input) {
                    "+" => false,
                    "-" => true,
                    _ => unreachable!(),
                }
                _ => unreachable!(),
            };

            let (r, i): (_, i32) = match tokens.next().unwrap() {
                s @ Token::Symbol(_) => match tokens.next().unwrap() {
                    i @ Token::Integer(_) => (Register::from_str(s.as_str(input)).unwrap(), i.as_str(input).parse().unwrap()),
                    _ => unreachable!(),
                },
                i @ Token::Integer(_) => match tokens.next().unwrap() {
                    s @ Token::Symbol(_) => (Register::from_str(s.as_str(input)).unwrap(), i.as_str(input).parse().unwrap()),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            assert!(tokens.next().unwrap().closerp());
            // TODO
            if negate {
                (r, -i)
            } else {
                (r, i)
            }
        },
        _ => unreachable!(),
    }
}

macro_rules! i2 {
    ($op:ident, $asm:expr, $tokens:ident, $input:ident) => {
        {
            let rd = unwrap_register($tokens, $input);
            let (rs1, imm) = offset($tokens, $input);
            assert!($tokens.next().unwrap().closerp());
            $asm.$op(rd, rs1, imm);
        }
    };
}

macro_rules! s {
    ($op:ident, $asm:expr, $tokens:ident, $input:ident) => {
        {
            let (rs1, imm) = offset($tokens, $input);
            let rd = unwrap_register($tokens, $input);
            assert!($tokens.next().unwrap().closerp());
            $asm.$op(rd, rs1, imm);
        }
    };
}

macro_rules! b {
    ($op:ident, $asm:expr, $tokens:ident, $input:ident) => {
        {
            let rs1 = unwrap_register($tokens, $input);
            let rs2 = unwrap_register($tokens, $input);
            let label = if let Some(Token::Symbol(i)) = $tokens.next() {
                Token::Symbol(*i).as_str($input)
            } else {
                unreachable!();
            };
            assert!($tokens.next().unwrap().closerp());
            $asm.$op(rs1, rs2, label);
        }
    };
}

impl<'a> Asm<'a> {
    fn assemble(tokens: Vec<Token>, input: &'a str) -> (Vec<u8>, Vec<Vec<u8>>, HashMap<usize, usize>) {
        let mut asm = Asm {
            asm: Assembler::new(),
            data: Vec::new(),
            rewrites: HashMap::new(),
            constants: HashMap::new(),
            globals: HashMap::new(),
        };

        let mut tokens = tokens.iter().filter(|t| !t.commentp());
        while let Some(t) = tokens.next() {
            match t {
                s @ Token::Symbol(_) => { asm.asm.label(s.as_str(input)); },
                Token::LParen(_) => if let Some(Token::Symbol(i)) = tokens.next() {
                    let s = Token::Symbol(*i).as_str(input);
                    if "define" == s {
                        asm.handle_define(&mut tokens, input);
                    } else {
                        asm.handle_opcode(s, &mut tokens, input);
                    }
                } else {
                    unreachable!();
                },
                _ => unreachable!(),
            }
        }

        (asm.asm.finish(), asm.data, asm.rewrites)
    }

    fn handle_define<'b,  I: Iterator<Item = &'b Token>>(&mut self, tokens: &mut I, input: &'a str) {
        let var = if let Some(Token::Symbol(i)) = tokens.next() {
            Token::Symbol(*i).as_str(input)
        } else {
            unreachable!();
        };

        match tokens.next().unwrap() {
            s @ Token::Integer(_) => {
                let i = s.as_str(input).parse().unwrap();
                self.constants.insert(var, i);
            }
            s @ Token::Char(_) => {
                let s = s.as_str(input);
                let s = &s[2..(s.len() - 1)].as_bytes();
                let i = match s[0] {
                    b'\\' => match s[1] {
                        b'\\' | b'\'' => s[1] as i32,
                        b'r' => b'\r' as i32,
                        b'n' => b'\n' as i32,
                        b't' => b'\t' as i32,
                        b'0' => b'\0' as i32,
                        _ => unreachable!(),
                    },
                    _ => s[0] as i32,
                };
                self.constants.insert(var, i);
            }
            s @ Token::String(_) => {
                let s = s.as_str(input).as_bytes();
                let mut v = Vec::with_capacity(s.len());
                let mut i = 0;
                while i < s.len() {
                    match s[i] {
                        b'\\' => {
                            v.push(match s[i+1] {
                                b'"' | b'\\' => s[i+1],
                                b'r' => b'\r',
                                b'n' => b'\n',
                                b't' => b'\t',
                                b'0' => b'\0',
                                _ => unreachable!(),
                            });
                            i += 1;
                        }
                        _ => v.push(s[i]),
                    }
                    i += 1;
                }
                self.globals.insert(var, self.data.len());
                self.data.push(s[1..s.len()-1].to_vec());
            }
            _ => unreachable!(),
        }

        assert!(tokens.next().unwrap().closerp());
    }

    fn read_imm<'b, I: Iterator<Item = &'b Token>>(&mut self, tokens: &mut I, input: &str) -> i32 {
        match tokens.next().unwrap() {
            s @ Token::Integer(_) => s.as_str(input).parse::<i32>().unwrap(),
            s @ Token::Char(_) => {
                let s = s.as_str(input);
                let s = &s[2..(s.len() - 1)].as_bytes();
                match s[0] {
                    b'\\' => match s[1] {
                        b'"' | b'\\' | b'\'' => s[1] as i32,
                        b'r' => b'\r' as i32,
                        b'n' => b'\n' as i32,
                        b't' => b'\t' as i32,
                        b'0' => b'\0' as i32,
                        _ => unreachable!(),
                    },
                    _ => s[0] as i32,
                }
            }
            s @ Token::Symbol(_) => {
                let s = s.as_str(input);
                *self.constants.get(s).unwrap()
            },
            s @ _ => unreachable!("{:?}", s),
        }
    }

    fn handle_opcode<'b, I: Iterator<Item = &'b Token>>(&mut self, opcode: &str, tokens: &mut I, input: &str) {
        match opcode {
            "add" => r!(add, self.asm, tokens, input),
            "sub" => r!(sub, self.asm, tokens, input),
            "xor" => r!(xor, self.asm, tokens, input),
            "or" => r!(or, self.asm, tokens, input),
            "and" => r!(and, self.asm, tokens, input),
            "sll" => r!(sll, self.asm, tokens, input),
            "srl" => r!(srl, self.asm, tokens, input),
            "sra" => r!(sra, self.asm, tokens, input),
            "slt" => r!(slt, self.asm, tokens, input),
            "sltu" => r!(sltu, self.asm, tokens, input),

            "addi" => i!(addi, self, tokens, input),
            "subi" => i!(subi, self, tokens, input),
            "xori" => i!(xori, self, tokens, input),
            "ori" => i!(ori, self, tokens, input),
            "andi" => i!(andi, self, tokens, input),
            "slli" => i!(slli, self, tokens, input),
            "srli" => i!(srli, self, tokens, input),
            "srai" => i!(srai, self, tokens, input),
            "slti" => i!(slti, self, tokens, input),
            "sltiu" => i!(sltiu, self, tokens, input),

            "lb" => i2!(lb, self.asm, tokens, input),
            "lh" => i2!(lh, self.asm, tokens, input),
            "lw" => i2!(lw, self.asm, tokens, input),
            "ld" => i2!(ld, self.asm, tokens, input),
            "lbu" => i2!(lbu, self.asm, tokens, input),
            "lhu" => i2!(lhu, self.asm, tokens, input),
            "lwu" => i2!(lwu, self.asm, tokens, input),
            "la" => {
                let rd = unwrap_register(tokens, input);
                // TODO: offsets?
                let symbol = if let Some(Token::Symbol(i)) = tokens.next() {
                    Token::Symbol(*i).as_str(input)
                } else {
                    unreachable!();
                };
                assert!(tokens.next().unwrap().closerp());
                let addr = self.globals.get(symbol).unwrap();
                self.rewrites.insert(self.asm.len(), *addr);
                self.asm.lui(rd, 0);
                self.asm.addi(rd, rd, 0);
            }

            "sb" => s!(sb, self.asm, tokens, input),
            "sh" => s!(sh, self.asm, tokens, input),
            "sw" => s!(sw, self.asm, tokens, input),
            "sd" => s!(sd, self.asm, tokens, input),

            "beq" => b!(beq, self.asm, tokens, input),
            "bne" => b!(bne, self.asm, tokens, input),
            "blt" => b!(blt, self.asm, tokens, input),
            "bge" => b!(bge, self.asm, tokens, input),
            "bltu" => b!(bltu, self.asm, tokens, input),
            "bgeu" => b!(bgeu, self.asm, tokens, input),

            "jal" => {
                let rd = unwrap_register(tokens, input);
                let label = if let Some(Token::Symbol(i)) = tokens.next() {
                    Token::Symbol(*i).as_str(input)
                } else {
                    unreachable!();
                };
                assert!(tokens.next().unwrap().closerp());
                self.asm.jal(rd, label);
            }
            "jalr" => {
                let rd = unwrap_register(tokens, input);
                let (rs, imm) = offset(tokens, input);
                assert!(tokens.next().unwrap().closerp());
                // TODO
                self.asm.jalr(rd, rs, imm);
            }
            "lui" => {
                let rd = unwrap_register(tokens, input);
                let imm = self.read_imm(tokens, input);
                assert!(tokens.next().unwrap().closerp());
                self.asm.lui(rd, imm as u32);
            }
            "auipc" => {
                let rd = unwrap_register(tokens, input);
                let imm = self.read_imm(tokens, input);
                assert!(tokens.next().unwrap().closerp());
                self.asm.auipc(rd, imm as u32);
            }
            "ecall" => {
                assert!(tokens.next().unwrap().closerp());
                self.asm.ecall();
            }
            "ebreak" => {
                assert!(tokens.next().unwrap().closerp());
                self.asm.ebreak();
            }
            _ => unreachable!("{}", opcode),
        }
    }
}
