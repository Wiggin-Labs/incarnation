use assembly::{Instruction, Operand};

use asm::{Assembler, Register};

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Opcode,
    ZeroOperands,
    Register,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::Opcode => "Invalid opcode",
            Error::ZeroOperands => "Expected no operands",
            Error::Register => "Invalid register name",
        })
    }
}

pub fn assemble(instructions: Vec<Instruction>, input: &str) -> Result<Vec<u8>, Error> {
    let mut asm = Assembler::new();

    for instruction in instructions {
        match instruction.opcode.as_str(input) {
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

                        let from = match from {
                            Operand::Constant(t) => {
                                // TODO
                                //t.as_i32(input)
                                t.as_str(input).parse::<i32>().unwrap()
                            }
                            // TODO
                            _ => unimplemented!(),
                        };
                        asm.mov_reg_i32(register, from);
                    }
                    // TODO
                    _ => unimplemented!(),
                }
            },
            "syscall" => {
                if instruction.operands.len() != 0 {
                    return Err(Error::ZeroOperands);
                }
                asm.syscall();
            }
            _ => return Err(Error::Opcode),
        }
    }

    Ok(asm.finish())
}
