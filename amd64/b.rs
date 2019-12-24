extern crate parser;

use parser::Token;

use std::env;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let mut output = String::new();

    let mut input = String::new();
    let mut f = File::open("instructions.data").unwrap();
    f.read_to_string(&mut input).unwrap();
    let tokens = parser::Tokenizer::tokenize(&input).unwrap();

    write!(output, "impl Assembler {{\n").unwrap();

    parse(tokens, &input, &mut output);

    write!(output, "}}").unwrap();

    {
        use std::io::Write;
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("instructions.rs");
        let mut f = File::create(&dest_path).unwrap();
        f.write_all(output.as_bytes()).unwrap();
    }
}

enum Operand {
    Imm8,
    Imm16,
    Imm32,
    Imm64,
    Reg8,
    Reg16,
    Reg32,
    Reg64,
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        f.write_str(match *self {
            Imm8 => "imm8",
            Imm16 => "imm16",
            Imm32 => "imm32",
            Imm64 => "imm64",
            Reg8 => "reg8",
            Reg16 => "reg16",
            Reg32 => "reg32",
            Reg64 => "reg64",
        })
    }
}

impl From<&str> for Operand {
    fn from(s: &str) -> Operand {
        use Operand::*;
        match s.to_lowercase().as_str() {
            "imm8" => Imm8,
            "imm16" => Imm16,
            "imm32" => Imm32,
            "imm64" => Imm64,
            "reg8" => Reg8,
            "reg16" => Reg16,
            "reg32" => Reg32,
            "reg64" => Reg64,
            _ => panic!(""),
        }
    }
}

fn parse(tokens: Vec<Token>, input: &str, output: &mut String) {
    use Token::*;
    let mut instructions = Vec::new();
    let mut position = 0;
    while position < tokens.len() {
        if !tokens[position].openerp() {
            panic!("");
        }
        let opener = tokens[position];

        position += 1;
        if !tokens[position].is_symbol() {
            panic!("");
        }

        let instruction = tokens[position].as_str(input);

        position += 1;
        if !tokens[position].openerp() {
            panic!("");
        }
        let mut operands = Vec::new();

        {
            let opener = tokens[position];
            position += 1;

            while !tokens[position].closerp() {
                if !tokens[position].is_symbol() {
                    panic!("");
                }
                let operand = tokens[position].as_str(input);
                operands.push(Operand::from(operand));
                position += 1;
            }
            assert!(opener.opener_match(tokens[position]));
            position += 1;
        }
        assert!(tokens[position].closerp());
        assert!(opener.opener_match(tokens[position]));

        instructions.push((instruction, operands));
        position += 1;
    }

    for (instruction, operands) in instructions {
        write!(output, "    pub fn {}", instruction.to_lowercase()).unwrap();
        for op in operands {
            write!(output, "_{}", op).unwrap();
        }
        write!(output, "() {{\n").unwrap();
        write!(output, "}}\n\n").unwrap();
    }
}
