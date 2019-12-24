extern crate amd64;
extern crate asm_syntax;
extern crate incarnation;
extern crate tokenizer;

//use incarnation::{amd64, assembly};

fn main() {
    let input = r"
    ;; print 'H'
    (mov rax (i32 1))
    ;; stdout
    (mov rdi (i32 1))
    ;; our string is the stack pointer
    ;; write 'Hello, world!\n' to the stack
    ;; \n
    (mov (address rsp) (u8 10))
    (sub rsp (u8 1))
    ;; !
    (mov (address rsp) (u8 33))
    (sub rsp (u8 1))
    ;; d
    (mov (address rsp) (u8 100))
    (sub rsp (u8 1))
    ;; l
    (mov (address rsp) (u8 108))
    (sub rsp (u8 1))
    ;; r
    (mov (address rsp) (u8 114))
    (sub rsp (u8 1))
    ;; o
    (mov (address rsp) (u8 111))
    (sub rsp (u8 1))
    ;; w
    (mov (address rsp) (u8 119))
    (sub rsp (u8 1))
    ;; ' '
    (mov (address rsp) (u8 32))
    (sub rsp (u8 1))
    ;; ,
    (mov (address rsp) (u8 44))
    (sub rsp (u8 1))
    ;; o
    (mov (address rsp) (u8 111))
    (sub rsp (u8 1))
    ;; l
    (mov (address rsp) (u8 108))
    (sub rsp (u8 1))
    ;; l
    (mov (address rsp) (u8 108))
    (sub rsp (u8 1))
    ;; e
    (mov (address rsp) (u8 101))
    (sub rsp (u8 1))
    ;; H
    (mov (address rsp) (u8 72))
    (mov rsi rsp)
    ;; length
    (mov rdx (i32 14))
    (syscall)

    ;; Call exit syscall with exitcode 0
    (mov rax (i32 231))
    (mov rdi (i32 0))
    (syscall)
    ";
    let tokens = tokenizer::Tokenizer::tokenize(input).unwrap();
    let instructions = asm_syntax::parser::parse(&tokens, input, false).unwrap();
    //let instructions = assembly::parse(&tokens, input, false).unwrap();
    let code = amd64::assemble(instructions, input).unwrap();
    incarnation::executable::generate_executable("test.o".into(), code);
}
