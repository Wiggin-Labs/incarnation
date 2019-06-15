extern crate asm;
extern crate incarnation;
use asm::{Assembler, Register};

fn main() {
    let mut asm = Assembler::new();
    asm.mov_reg_i32(Register::Rax, 231);
    asm.mov_reg_i32(Register::Rdi, 42);
    asm.syscall();
    let code = asm.finish();

    incarnation::executable::generate_executable("test.o".into(), code);
}
