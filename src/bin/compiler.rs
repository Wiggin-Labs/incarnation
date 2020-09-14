extern crate amd64;
extern crate asm_syntax;
extern crate incarnation;
extern crate parser;
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
    println!("Ok");
}
