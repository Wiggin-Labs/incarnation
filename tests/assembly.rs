extern crate asm;
extern crate incarnation;

use asm::{Assembler, Register};
use incarnation::{amd64, assembly, parser};

#[test]
pub fn basic() {
    let input = r"
    (mov rax 231)
    (mov rdi 42)
    (syscall)
    ";

    let tokens = parser::Tokenizer::tokenize(input).unwrap();
    let instructions = assembly::parse(&tokens, input, false).unwrap();
    let expected = {
        let mut asm = Assembler::new();
        asm.mov_reg_i32(Register::Rax, 231);
        asm.mov_reg_i32(Register::Rdi, 42);
        asm.syscall();
        asm.finish()
    };
    assert_eq!(expected, amd64::assemble(instructions, input).unwrap());
}
