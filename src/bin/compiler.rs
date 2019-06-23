extern crate incarnation;

use incarnation::{amd64, assembly, parser};

fn main() {
    let input = r"
    ;; print 'H'
    (mov rax 1)
    ;; stdout
    (mov rdi 1)
    ;; our string is the stack pointer
    ;; write 'Hello, world!\n' to the stack
    ;; \n
    (mov-u8 (address rsp) 10)
    (sub rsp 1)
    ;; !
    (mov-u8 (address rsp) 33)
    (sub rsp 1)
    ;; d
    (mov-u8 (address rsp) 100)
    (sub rsp 1)
    ;; l
    (mov-u8 (address rsp) 108)
    (sub rsp 1)
    ;; r
    (mov-u8 (address rsp) 114)
    (sub rsp 1)
    ;; o
    (mov-u8 (address rsp) 111)
    (sub rsp 1)
    ;; w
    (mov-u8 (address rsp) 119)
    (sub rsp 1)
    ;; ' '
    (mov-u8 (address rsp) 32)
    (sub rsp 1)
    ;; ,
    (mov-u8 (address rsp) 44)
    (sub rsp 1)
    ;; o
    (mov-u8 (address rsp) 111)
    (sub rsp 1)
    ;; l
    (mov-u8 (address rsp) 108)
    (sub rsp 1)
    ;; l
    (mov-u8 (address rsp) 108)
    (sub rsp 1)
    ;; e
    (mov-u8 (address rsp) 101)
    (sub rsp 1)
    ;; H
    (mov-u8 (address rsp) 72)
    (mov rsi rsp)
    ;; length
    (mov rdx 14)
    (syscall)

    ;; Call exit syscall with exitcode 0
    (mov rax 231)
    (mov rdi 0)
    (syscall)
    ";
    let tokens = parser::Tokenizer::tokenize(input).unwrap();
    let instructions = assembly::parse(&tokens, input, false).unwrap();
    let code = amd64::assemble(instructions, input).unwrap();
    incarnation::executable::generate_executable("test.o".into(), code);
}
