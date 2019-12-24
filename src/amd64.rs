use assembly::{Instruction, Operand};

use asm::{Assembler, Register};

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Opcode,
    ZeroOperands,
    Register,
    ToConstant,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::Opcode => "Invalid opcode",
            Error::Operands => "Incorrect number of operands",
            Error::Register => "Invalid register name",
            Error::ToConstant => "Cannot mov to a constant",
        })
    }
}

pub fn assemble(instructions: Vec<Instruction>, input: &str) -> Result<Vec<u8>, Error> {
    let mut asm = Assembler::new();

    for instruction in instructions {
        match instruction.opcode.as_str(input) {
            /*
            "mov-u8" => {
                // TODO
                assert!(instruction.operands.len() == 2);
                let to = instruction.operands[0];
                let from = instruction.operands[1];
                match to {
                    Operand::Register(t) => {
                    }
                    Operand::Address(t) => {
                        let register = if let Some(r) = Register::from_str(t.as_str(input)) {
                            r
                        } else {
                            return Err(Error::Register);
                        };

                        match from {
                            Operand::Constant(t) => {
                                // TODO
                                //t.as_i32(input)
                                let from = t.as_str(input).parse::<u8>().unwrap();
                                // TODO
                                asm.mov_addr_u8(register, from);
                            }
                            // TODO
                            Operand::Register(t) => {
                                unimplemented!()
                            }
                            // TODO
                            Operand::Address(_) => unreachable!(),
                        }
                    }
                    Operand::Constant(_) => return Err(Error::ToConstant),
                }
            }
            "mov" => {
                // TODO
                assert!(instruction.operands.len() == 2);
                let to = instruction.operands[0];
                let from = instruction.operands[1];
                match to {
                    Operand::Register(t) => {
                        let register = if let Some(r) = Register::from_str(t.as_str(input)) {
                            r
                        } else {
                            return Err(Error::Register);
                        };

                        match from {
                            Operand::Constant(t) => {
                                // TODO
                                //t.as_i32(input)
                                let from = t.as_str(input).parse::<i32>().unwrap();
                                // TODO
                                asm.mov_reg_i32(register, from);
                            }
                            Operand::Register(t) => {
                                if let Some(r) = Register::from_str(t.as_str(input)) {
                                    asm.mov_reg_reg(register, r);
                                } else {
                                    return Err(Error::Register);
                                }
                            }
                            // TODO
                            _ => unimplemented!(),
                        }
                    }
                    Operand::Address(t) => {
                        let register = if let Some(r) = Register::from_str(t.as_str(input)) {
                            r
                        } else {
                            return Err(Error::Register);
                        };

                        match from {
                            Operand::Constant(t) => {
                                // TODO
                                //t.as_i32(input)
                                let from = t.as_str(input).parse::<i32>().unwrap();
                                // TODO
                                asm.mov_addr_i32(register, from);
                            }
                            // TODO
                            Operand::Register(t) => {
                                unimplemented!()
                            }
                            // TODO
                            Operand::Address(_) => unreachable!(),
                        }
                    }
                    Operand::Constant(_) => return Err(Error::ToConstant),
                }
            },
            */
            "syscall" => {
                if instruction.operands.len() != 0 {
                    return Err(Error::Operands);
                }
                asm.syscall();
            }
        /*
            "sub" => {
                // TODO
                assert!(instruction.operands.len() == 2);
                let to = instruction.operands[0];
                let from = instruction.operands[1];
                if let Operand::Register(t) = to {
                    let t = Register::from_str(t.as_str(input)).unwrap();
                    if let Operand::Constant(c) = from {
                        let c = c.as_str(input).parse::<u8>().unwrap();
                        asm.sub_reg_u8(t, c);
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            }
        */
            _ => return Err(Error::Opcode),
        }
    }

    Ok(asm.finish())
}
