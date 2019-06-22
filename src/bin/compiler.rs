extern crate incarnation;

use incarnation::{amd64, assembly, parser};

fn main() {
    let input = r"
    (mov rax 231)
    (mov rdi 42)
    (syscall)
    ";
    let tokens = parser::Tokenizer::tokenize(input).unwrap();
    let instructions = assembly::parse(&tokens, input, false).unwrap();
    let code = amd64::assemble(instructions, input).unwrap();
    incarnation::executable::generate_executable("test.o".into(), code);
}
