extern crate asm_syntax;
extern crate byteorder;
extern crate string_interner;

#[macro_use]
mod macros;

mod aso;
mod assembler;
mod emitter;
mod modrm;
mod oso;
mod register;
mod rex;
mod sib;

use emitter::Emitter;

use asm_syntax::parser::{Immediate, Instruction, Operand};
use string_interner::{INTERNER, Symbol};

use std::fmt::{self, Display, Formatter};

pub use aso::ASO;
pub use assembler::Assembler;
pub use modrm::ModRM;
pub use oso::OSO;
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

fn symbol_value(s: Symbol) -> String {
    INTERNER.lock().unwrap().get_value(s).unwrap()
}

pub fn assemble(instructions: Vec<Instruction>) -> Result<Vec<u8>, Error> {
    let mut asm = Assembler::new();

    for instruction in instructions {
        match symbol_value(instruction.opcode).as_str() {
            "mov" => {
                assert!(instruction.operands.len() == 2);

                let to = instruction.operands[0];
                let from = instruction.operands[1];

                let to_register = if to.is_constant() {
                    return Err(Error::ToConstant);
                } else {
                    let register = to.unwrap_register();
                    if let Some(r) = Register::from_str(&symbol_value(register)) {
                        r
                    } else {
                        return Err(Error::Register);
                    }
                };

                match from {
                    // TODO
                    Operand::Register(t) | Operand::Address(_, t) => {
                        let from_register = if let Some(r) = Register::from_str(&symbol_value(t)) {
                            r
                        } else {
                            return Err(Error::Register);
                        };

                        match (to.is_address(), from.is_address()) {
                            //(true, true) =>
                            //    asm.mov_addr_addr(to_register, to.unwrap_disp(),
                            //                      from_register, from.unwrap_disp()),
                            (true, false) =>
                                asm.mov_addr_reg(to_register, from_register, to.unwrap_disp()),
                            (false, true) =>
                                asm.mov_reg_addr(to_register, from_register, from.unwrap_disp()),
                            (false, false) => asm.mov_reg_reg(to_register, from_register),
                            _ => todo!(),
                        }
                    }
                    //Operand::Address(t) => {
                    //}
                    Operand::Constant(ty, t) => {
                        let imm = Immediate::from(&symbol_value(t), ty)?;

                        if to.is_address() {
                            asm.mov_addr_imm(to_register, None, imm);
                        } else {
                            asm.mov_reg_imm(to_register, imm);
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
                    let t = Register::from_str(&symbol_value(t)).unwrap();
                    if let Operand::Constant(_ty, c) = from {
                        let c = symbol_value(c).parse::<u8>().unwrap();
                        asm.sub_reg_u8(t, c);
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            }
            "push" => {
                assert!(instruction.operands.len() == 1);
                let from = instruction.operands[0];
                if let Operand::Register(t) = from {
                    let from = Register::from_str(&symbol_value(t)).unwrap();
                    asm.push_reg(from);
                } else {
                    unreachable!();
                }
            }
            "pop" => {
                assert!(instruction.operands.len() == 1);
                let to = instruction.operands[0];
                if let Operand::Register(t) = to {
                    let to = Register::from_str(&symbol_value(t)).unwrap();
                    asm.pop_reg(to);
                } else {
                    unreachable!();
                }
            }
            _ => return Err(Error::Opcode),
        }
    }

    Ok(asm.finish())
}
