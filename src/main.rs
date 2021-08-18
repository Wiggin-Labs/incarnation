extern crate amd64;
extern crate asm_syntax;
extern crate parser;
extern crate string_interner;
extern crate tokenizer;
extern crate type_checker;

fn main() {
    /*
    let tokens = tokenizer::Tokenizer::tokenize(input).unwrap();
    let instructions = asm_syntax::parser::parse(&tokens, input, false).unwrap();
    //let instructions = assembly::parse(&tokens, input, false).unwrap();
    let code = amd64::assemble(instructions, input).unwrap();
    incarnation::executable::generate_executable("test.o".into(), code);
    */

    /*
    let input = r#"
    (include libs/unix/lib.inc)
    (defn (main ())
      (exit 5))
    "#;
    let tokens = tokenizer::Tokenizer::tokenize(input).unwrap();
    let ast = parser::parse(tokens, input).unwrap();

    let input = include_str!("../../libs/unix/lib.inc");
    let tokens = tokenizer::Tokenizer::tokenize(input).unwrap();
    let ast = parser::parse(tokens, input).unwrap();
    type_checker::type_check(ast).unwrap();
    */

    /*
    let fizzbuzz = r#"
    (defn (main ())
        (fizzbuzz 1 100)
        (exit 0))

    (defn (print ([s string])))
    (defn (exit ([a i32])))

    (defn (<= ([a i32] [b i32]) bool)
        #t)

    (defn (+ ([a i32] [b i32]) i32)
        0)

    (defn (% ([a i32] [b i32]) i32)
        0)

    (defn (= ([a i32] [b i32]) bool)
        #t)

    (defn (fizzbuzz ([i i32] [n i32]))
        (if (<= i n)
            {begin
                (if (= 0 (% i 15))
                    (print "fizzbuzz\n")
                    (if (= 0 (% i 5))
                        (print "buzz\n")))
                        #|(if (= 0 (% i 3))
                            (print "fizz\n"))))|#
                (fizzbuzz (+ i 1) n)}))
    "#;

    let tokens = tokenizer::Tokenizer::tokenize(fizzbuzz).unwrap();
    let ast = parser::parse(tokens, fizzbuzz).unwrap();
    */
    let input = r#"
    (defn (main ())
        (print "hello, world!\n")
        (exit 0))

    (define STDOUT 1)

    (defn (print ([s string] [length usize]))
        (write STDOUT s length))

    (defn (write ([fd i32] [data string] [size usize]))
        (#asm (mov rax (i32 1))
              (syscall)))

    (defn (exit ([i i32]))
        (#asm (mov rax (i32 0))
              (syscall)))
    "#;
    let tokens = tokenizer::Tokenizer::tokenize(input).unwrap();
    let ast = parser::parse(tokens, input).unwrap();
    type_checker::type_check(&ast).unwrap();
    compile(ast, input);
    //let instructions = compile(ast);
    //let code = amd64::assemble(instructions).unwrap();
    println!("Ok");
}

fn compile(mut ast: Vec<Ast>, input: &str) {
    // If we do this we cover definitions of constants prior to compiling procedures
    ast.sort_unstable_by(|a, b|
        if a.is_define() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        });

    for a in ast {
        match a {
            Ast::Include(_) => todo!(),
            Ast::Define { name, value, .. } => match *value {
                Ast::Primitive(p) => {
                    ir.push(Lir::Constant(name, match p {
                        CompilePrimitive::Bool(b) => Primitive::Bool(b),
                        CompilePrimitive::Integer(i) => Primitive::I32(i),
                        CompilePrimitive::String(s) => Primitive::String(s),
                    }));
                }
                //Ast::Primitive(_) | Ast::Identifier(_) => { constants.insert(name, *value); }
                _ => unimplemented!(),
            },
            Ast::Defn { name, args, body, .. } => compile_defn(name, args, body, input, &mut constants, &mut code),
            _ => unreachable!(),
        }
    }
}

fn compile_defn(name: Symbol, args: Vec<Arg>, body: Vec<Ast>, input: &str) -> Lir {
    if body.len() == 1 && body[0].is_asm() {
        todo!();
    }
    Lir::Fun(name, _)
}

/*
use asm_syntax::{Immediate, Instruction, Operand};
use parser::{Arg, Ast, CompilePrimitive};
use string_interner::{get_symbol, Symbol};

use std::collections::HashMap;

fn compile(mut ast: Vec<Ast>, input: &str) {
    //let mut instructions = Vec::new();
    let mut constants = HashMap::new();
    let mut code = HashMap::new();

    // If we do this we cover definitions of constants prior to compiling procedures
    ast.sort_unstable_by(|a, b|
        if a.is_define() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        });

    for a in ast {
        match a {
            Ast::Include(_) => todo!(),
            Ast::Define { name, value, .. } => match *value {
                Ast::Primitive(_) | Ast::Identifier(_) => { constants.insert(name, *value); }
                _ => unimplemented!(),
            },
            Ast::Defn { name, args, body, .. } => compile_defn(name, args, body, input, &mut constants, &mut code),
            _ => unreachable!(),
        }
    }

    //instructions
}

fn compile_defn(name: Symbol, args: Vec<Arg>, body: Vec<Ast>, input: &str,
                constants: &mut HashMap<Symbol, Ast>,
                code: &mut HashMap<Symbol, Vec<Instruction>>)
{
    let mut instructions = Vec::new();
    let arguments: Vec<_> = args.iter().map(|x| x.name).collect();
    let arguments = {
        let mut h = HashMap::new();
        for (i, a) in arguments.iter().enumerate() {
            h.insert(a, get_register(i));
        }
        h
    };

    let body_len = body.len();
    for a in body {
        match a {
            Ast::Asm(t) => {
                assert!(body_len == 1);
                instructions = asm_syntax::parser::parse(&t, input, true).unwrap();
            }
            Ast::Application(v) => {
                let args = &v[1..];
                if args.len() > 6 {
                    panic!("only 6 args supported");
                }

                // We need to process non-immediate values first.
                let mut args = args.iter().enumerate().collect::<Vec<_>>();
                args.sort_unstable_by(|a, b|
                    if a.1.is_if() || a.1.is_application() || a.1.is_block() {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    });

                for (i, a) in args {
                    match a {
                        Ast::Identifier(ident) => {
                            // TODO
                            let current_register = *arguments.get(ident).unwrap();
                            if current_register != get_register(i) {
                                instructions.push(Instruction::new(get_symbol("mov"),
                                    vec![Operand::Register(get_register(i)),
                                         Operand::Register(current_register)]));
                            }

                        }
                        Ast::Primitive(p) => match p {
                            CompilePrimitive::Integer(n) =>
                                instructions.push(Instruction::new(get_symbol("mov"),
                                    vec![Operand::Register(get_register(i)),
                                    // TODO: have other types
                                         Operand::Constant(Immediate::U64(*n as u64))])),
                            CompilePrimitive::Bool(b) =>
                                instructions.push(Instruction::new(get_symbol("mov"),
                                    vec![Operand::Register(get_register(i)),
                                         Operand::Constant(Immediate::U8(*b as u8))])),
                            CompilePrimitive::String(s) => {
                                // TODO
                                instructions.push(Instruction::new(get_symbol("mov"),
                                    vec![Operand::Register(get_register(i)),
                                         Operand::Constant(Immediate::U64(0))])),
                            }
                        },
                        _ => unimplemented!(),
                    }
                }

                if let Ast::Identifier(i) = v[0] {
                    instructions.push(Instruction::new(get_symbol("call"), vec![Operand::Label(i)]));
                } else {
                    todo!()
                }
            }
            _ => unimplemented!(),
        }
    }

    code.insert(name, instructions);
}

fn get_register(i: usize) -> Symbol {
    get_symbol(match i {
        0 => "rdi",
        1 => "rsi",
        2 => "rdx",
        3 => "rcx",
        4 => "r8",
        5 => "r9",
        _ => unimplemented!(),
    })
}
*/
