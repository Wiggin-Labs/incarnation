extern crate asm_syntax;
extern crate byteorder;

mod assembler;
mod emitter;
#[macro_use]
mod macros;
mod modrm;
mod register;
mod rex;
mod sib;

use emitter::Emitter;

use asm_syntax::parser::{Constant, Instruction, Operand};

use std::fmt::{self, Display, Formatter};

pub use assembler::Assembler;
pub use modrm::ModRM;
pub use register::Register;
pub use rex::REX;
pub use sib::SIB;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Opcode,
    Operands,
    Register,
    ToConstant,
    InvalidConstant,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_err: std::num::ParseIntError) -> Error {
        Error::InvalidConstant
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::Opcode => "Invalid opcode",
            Error::Operands => "Incorrect number of operands",
            Error::Register => "Invalid register name",
            Error::ToConstant => "Cannot mov to a constant",
            Error::InvalidConstant => "Unable to parse constant to expected type",
        })
    }
}

pub fn assemble(instructions: Vec<Instruction>, input: &str) -> Result<Vec<u8>, Error> {
    let mut asm = Assembler::new();

    for instruction in instructions {
        match instruction.opcode.as_str(input) {
            "mov" => {
                assert!(instruction.operands.len() == 2);

                let to = instruction.operands[0];
                let from = instruction.operands[1];

                let to_register = if to.is_constant() {
                    return Err(Error::ToConstant);
                } else {
                    let register = to.unwrap_register();
                    if let Some(r) = Register::from_str(register.as_str(input)) {
                        r
                    } else {
                        return Err(Error::Register);
                    }
                };

                match from {
                    // TODO
                    Operand::Register(t) | Operand::Address(_, t) => {
                        let from_register = if let Some(r) = Register::from_str(t.as_str(input)) {
                            r
                        } else {
                            return Err(Error::Register);
                        };

                        match (to.is_address(), from.is_address()) {
                            //(true, true) =>
                            //    asm.mov_addr_addr(to_register, to.unwrap_disp(),
                            //                      from_register, from.unwrap_disp()),
                            //(true, false) =>
                            //    asm.mov_addr_reg(to_register, to.unwrap_disp(), from_register),
                            //(false, true) =>
                            //    asm.mov_reg_addr(to_register, from_register, from.unwrap_disp()),
                            (false, false) => asm.mov_reg_reg(to_register, from_register),
                            _ => todo!(),
                        }
                    }
                    //Operand::Address(t) => {
                    //}
                    Operand::Constant(ty, t) => {
                        let t = t.as_str(input);
                        match ty {
                            Constant::U8 => if to.is_address() {
                                //asm.mov_addr_u8(to_register, to.unwrap_disp(), t.parse::<u8>()?);
                                asm.mov_addr_u8(to_register, t.parse::<u8>()?);
                            } else {
                                //asm.mov_reg_u8(to_register, t.parse::<u8>()?);
                            },
                            Constant::U16 => if to.is_address() {
                                //asm.mov_addr_u16(to_register, to.unwrap_disp(), t.parse::<u16>()?);
                            } else {
                                //asm.mov_reg_u16(to_register, t.parse::<u16>()?);
                            },
                            Constant::U32 => if to.is_address() {
                                //asm.mov_addr_u32(to_register, to.unwrap_disp(), t.parse::<u32>()?);
                            } else {
                                //asm.mov_reg_u32(to_register, t.parse::<u32>()?);
                            },
                            Constant::U64 => if to.is_address() {
                                //asm.mov_addr_u64(to_register, to.unwrap_disp(), t.parse::<u64>()?);
                            } else {
                                asm.mov_reg_u64(to_register, t.parse::<u64>()?);
                            },
                            Constant::I8 => if to.is_address() {
                                //asm.mov_addr_i8(to_register, to.unwrap_disp(), t.parse::<i8>()?);
                            } else {
                                //asm.mov_reg_i8(to_register, t.parse::<i8>()?);
                            },
                            Constant::I16 => if to.is_address() {
                                //asm.mov_addr_i16(to_register, to.unwrap_disp(), t.parse::<i16>()?);
                            } else {
                                //asm.mov_reg_i16(to_register, t.parse::<i16>()?);
                            },
                            Constant::I32 => if to.is_address() {
                                //asm.mov_addr_i32(to_register, to.unwrap_disp(), t.parse::<i32>()?);
                            } else {
                                asm.mov_reg_i32(to_register, t.parse::<i32>()?);
                            },
                            Constant::I64 => if to.is_address() {
                                //asm.mov_addr_i64(to_register, to.unwrap_disp(), t.parse::<i64>()?);
                            } else {
                                asm.mov_reg_i64(to_register, t.parse::<i64>()?);
                            },
                        }
                    }
                }
            }
            "syscall" => {
                if instruction.operands.len() != 0 {
                    return Err(Error::Operands);
                }
                asm.syscall();
            }
            "sub" => {
                // TODO
                assert!(instruction.operands.len() == 2);
                let to = instruction.operands[0];
                let from = instruction.operands[1];
                if let Operand::Register(t) = to {
                    let t = Register::from_str(t.as_str(input)).unwrap();
                    if let Operand::Constant(_ty, c) = from {
                        let c = c.as_str(input).parse::<u8>().unwrap();
                        asm.sub_reg_u8(t, c);
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            }
            _ => return Err(Error::Opcode),
        }
    }

    Ok(asm.finish())
}
