extern crate amd64;
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

#[test]
pub fn hello() {
    /*
    let input = r"
    (mov (address rsp) 72)
    ";
    */
    let input = r"
    ;; print 'H'
    (mov rax 1)
    ;; stdout
    (mov rdi 1)
    ;; write 'H' to the stack
    (mov (address rsp) 72)
    ;; our string is the stack pointer
    (mov rsi rsp)
    ;; length
    (mov rdx 1)
    (syscall)

    ;; Call exit syscall with exitcode 0
    (mov rax 231)
    (mov rdi 0)
    (syscall)
    ";

    let tokens = parser::Tokenizer::tokenize(input).unwrap();
    println!("{:?}", tokens);
    let instructions = assembly::parse(&tokens, input, false).unwrap();
    let expected = {
        let mut asm = Assembler::new();
        asm.mov_reg_i32(Register::Rax, 1);
        asm.mov_reg_i32(Register::Rdi, 1);
        asm.mov_addr_i32(Register::Rsp, 72);
        asm.mov_reg_reg(Register::Rsi, Register::Rsp);
        asm.mov_reg_i32(Register::Rdx, 1);
        asm.syscall();

        asm.mov_reg_i32(Register::Rax, 231);
        asm.mov_reg_i32(Register::Rdi, 0);
        asm.syscall();
        asm.finish()
    };
    let asm_expected = vec![
        0x48, 0xc7, 0xc0, 0x01, 0, 0, 0,
        0x48, 0x0c7, 0x0c7, 0x01, 0, 0, 0,
        0x48, 0x0c7, 0x04, 0x24, 0x048, 0, 0, 0,
        0x48, 0x89, 0xe6,
        0x48, 0xc7, 0xc2, 0x01, 0, 0, 0,
        0x0f, 0x05,
        0x48, 0xc7, 0xc0, 0xe7, 0, 0, 0,
        0x48, 0xc7, 0xc7, 0, 0, 0, 0,
        0x0f, 0x05,
    ];
    //assert_eq!(expected, asm_expected);
    assert_eq!(expected, amd64::assemble(instructions, input).unwrap());
}
