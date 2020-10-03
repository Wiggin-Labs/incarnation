extern crate amd64;
extern crate asm_syntax;
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

    let fizzbuzz = r#"
    (defn (main ())
        (fizzbuzz 1 100)
        (exit 0))

    (defn (print ([s string])))
    (defn (exit ([a i32])))

    (defn (le ([a i32] [b i32]) bool)
        #t)

    (defn (+ ([a i32] [b i32]) i32)
        0)

    (defn (% ([a i32] [b i32]) i32)
        0)

    (defn (= ([a i32] [b i32]) bool)
        #t)

    (defn (fizzbuzz ([i i32] [n i32]))
        (if (le i n)
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
    type_checker::type_check(ast).unwrap();
    println!("Ok");
}
