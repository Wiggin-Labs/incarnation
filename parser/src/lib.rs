#[macro_use]
extern crate derive_is_enum_variant;
extern crate string_interner;
extern crate tokenizer;

mod error;

pub use error::ParserError;

use string_interner::{INTERNER, Symbol};
use tokenizer::Token;

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Clone, Debug, is_enum_variant)]
pub enum Ast {
    Include(Symbol),
    Define {
        name: Symbol,
        ty: Type,
        value: Box<Ast>
    },
    Defn {
        name: Symbol,
        ty: Type,
        args: Vec<Arg>,
        body: Vec<Ast>,
    },
    Primitive(CompilePrimitive),
    Asm(Vec<Token>),
    Application(Vec<Ast>),
    Identifier(Symbol),
    /*
    Lambda {
        args: Vec<Arg>,
        ret_ty: Type,
        body: Vec<Ast>,
    }
    */
}

impl Ast {
    pub fn ty(&self) -> Type {
        use Ast::*;
        match self {
            Identifier(_) => Type::Hole,
            //Lambda { args, ret_ty, .. } =>
            //    Type::Arrow(args.iter().map(|arg| arg.ty.clone()).collect(), Box::new(ret_ty.clone())),
            Application(v) => v[0].ty(),
            Primitive(p) => p.ty(),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Arg {
    pub name: Symbol,
    pub ty: Type,
}

impl Arg {
    pub fn new(name: Symbol, ty: Type) -> Self {
        Arg {
            name: name,
            ty: ty,
        }
    }
}

#[derive(Clone, Debug, PartialEq, is_enum_variant)]
pub enum Type {
    U8,
    Usize,
    I32,
    Bool,
    String,
    Ptr(Box<Type>),
    /// Function type
    Arrow(Vec<Type>, Box<Type>),
    /// ()
    Empty,
    /// !
    Never,
    /// Type not known yet
    Hole,
}

impl Type {
    pub fn from_token(token: Token, input: &str, inner_ty: Option<Type>) -> Self {
        match token.as_str(input) {
            "u8" => Type::U8,
            "usize" => Type::Usize,
            "i32" => Type::I32,
            "!" => Type::Never,
            "ptr" if inner_ty.is_some() => Type::Ptr(Box::new(inner_ty.unwrap())),
            _ => todo!(),
        }
    }

    pub fn arrow_split(&self) -> (Vec<Self>, Self) {
        match self {
            Type::Arrow(args, ty) => (args.clone(), *ty.clone()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CompilePrimitive {
    Integer(i32),
    String(String),
    Bool(bool),
}

impl CompilePrimitive {
    pub fn ty(&self) -> Type {
        use CompilePrimitive::*;
        match self {
            Integer(_) => Type::I32,
            String(_) => Type::String,
            Bool(_) => Type::Bool,
        }
    }
}

pub fn parse(tokens: Vec<Token>, input: &str) -> Result<Vec<Ast>> {
    let mut ast = Vec::new();
    let mut tokens = Tokens::new(tokens);
    while tokens.peek().is_some() {
        if let Some(expr) = parse_expr(&mut tokens, input)? {
            match expr {
                Ast::Include(_) => ast.push(expr),
                Ast::Define { .. } => ast.push(expr),
                Ast::Defn { .. } => ast.push(expr),
                _ => return Err(ParserError::Item),
            }
        } else {
            return Err(ParserError::Closer);
        }
    }
    Ok(ast)
}

macro_rules! next {
    ( $token:ident, $tokens:ident, $body:expr) => {
        if let Some($token) = $tokens.next() {
            $body
        } else {
            return Err(ParserError::EOI);
        }
    };
}

macro_rules! get_symbol {
    ( $token:ident, $input:ident ) => {
        INTERNER.lock().unwrap().get_symbol($token.as_str($input).into())
    };
}


fn parse_expr(tokens: &mut Tokens, input: &str) -> Result<Option<Ast>> {
    next!(token, tokens, {
        match token {
            Token::Comment(_) | Token::BlockComment(_) => parse_expr(tokens, input),
            t if t.closerp() => Ok(None),
            t if t.openerp() => Ok(Some(parse_paren_expr(tokens, input)?)),
            t @ Token::Symbol(_) => Ok(Some(Ast::Identifier(get_symbol!(t, input)))),
            t @ Token::String(_) => Ok(Some(Ast::Primitive(CompilePrimitive::String(t.as_str(input).into())))),
            // TODO: need to check that this token fits an i32
            t @ Token::Integer(_) => Ok(Some(Ast::Primitive(CompilePrimitive::Integer(t.as_str(input).parse().unwrap())))),
            Token::Pound(_) => Ok(Some(parse_pound(tokens, input)?)),
            _ => todo!(),
        }
    })
}

fn parse_pound(tokens: &mut Tokens, input: &str) -> Result<Ast> {
    next!(token, tokens, {
        match token {
            t @ Token::Symbol(_) => match t.as_str(input) {
                // We should only encounter #asm at the beginning of a paren expression so this is
                // handled in parse_paren_expr
                "asm" => return Err(ParserError::Token),
                "t" => Ok(Ast::Primitive(CompilePrimitive::Bool(true))),
                "f" => Ok(Ast::Primitive(CompilePrimitive::Bool(false))),
                _ => todo!(),
            },
            _ => todo!(),
        }
    })
}

fn parse_paren_expr(tokens: &mut Tokens, input: &str) -> Result<Ast> {
    next!(token, tokens, {
        match token {
            t @ Token::Symbol(_) => return parse_identifier(t, tokens, input),
            Token::Pound(_) => if let Some(t) = tokens.peek() {
                if t.is_symbol() && t.as_str(input) == "asm" {
                    tokens.next();
                    handle_inline_asm(tokens, input)
                } else {
                    todo!()
                }
            } else {
                return Err(ParserError::EOI);
            },
            _ => todo!(),
        }
    })
}

fn handle_inline_asm(tokens: &mut Tokens, _input: &str) -> Result<Ast> {
    let mut asm = Vec::new();
    let mut openers = 1;
    while let Some(t) = tokens.next() {
        if t.openerp() {
            openers += 1;
        } else if t.closerp() {
            if openers == 1 {
                return Ok(Ast::Asm(asm));
            } else {
                openers -= 1;
            }
        }
        asm.push(t);
    }

    return Err(ParserError::EOI);
}

fn parse_identifier(t: Token, tokens: &mut Tokens, input: &str) -> Result<Ast> {
    match t.as_str(input) {
        "include" => handle_include(tokens, input),
        "define" => handle_define(tokens, input),
        "defn" => handle_defn(tokens, input),
        _ => handle_application(t, tokens, input),
    }
}

fn handle_define(tokens: &mut Tokens, input: &str) -> Result<Ast> {
    let name = next!(t, tokens, {
        if t.is_symbol() {
            get_symbol!(t, input)
        } else {
            return Err(ParserError::Token);
        }
    });

    let value = if let Some(expr) = parse_expr(tokens, input)? {
        expr
    } else {
        return Err(ParserError::Closer);
    };

    handle_closer(tokens)?;

    Ok(Ast::Define{
        name: name,
        ty: value.ty(),
        value: Box::new(value),
    })
}

fn handle_defn(tokens: &mut Tokens, input: &str) -> Result<Ast> {
    next!(token, tokens, {
        if !token.openerp() {
            return Err(ParserError::Token);
        }
    });

    let name = next!(token, tokens, {
        if token.is_symbol() {
            get_symbol!(token, input)
        } else {
            return Err(ParserError::Token);
        }
    });

    // Beginning of argument list
    next!(token, tokens, {
        if !token.openerp() {
            println!("{}", token.as_str(input));
            return Err(ParserError::Token);
        }
    });

    let mut args = Vec::new();
    // Each argument is of the form `(ident type)`
    loop {
        // Read the argument opener
        next!(token, tokens, {
            // A closer here denotes the end of the argument list.
            if token.closerp() {
                break;
            } else if !token.openerp() {
                return Err(ParserError::Token);
            }
        });

        let arg_name = next!(token, tokens, {
            if token.is_symbol() {
                get_symbol!(token, input)
            } else {
                return Err(ParserError::Token);
            }
        });

        let arg_type = read_type(tokens, input)?;

        // Read the argument closer
        handle_closer(tokens)?;

        args.push(Arg::new(arg_name, arg_type));
    }

    let ret_ty = if let Some(t) = tokens.peek() {
        if t.closerp() {
            Type::Empty
        } else {
            read_type(tokens, input)?
        }
    } else {
        return Err(ParserError::EOI);
    };

    // End of function preamble
    handle_closer(tokens)?;

    let mut body = Vec::new();
    while let Some(expr) = parse_expr(tokens, input)? {
        if expr.is_identifier() || expr.is_primitive() {
            if !tokens.peek().map(|t| t.closerp()).unwrap_or(false) {
                // TODO: this error should signify that an identifier or primitive can only exist
                // as the last item in a procedure
                return Err(ParserError::Token);
            }
        }
        body.push(expr);
    }

    let ty = Type::Arrow(args.iter().map(|arg| arg.ty.clone()).collect(), Box::new(ret_ty.clone()));
    Ok(Ast::Defn {
        name: name,
        ty: ty,
        args: args,
        body: body,
    })
}

fn read_type(tokens: &mut Tokens, input: &str) -> Result<Type> {
    next!(token, tokens, {
        if token.is_symbol() {
            Ok(Type::from_token(token, input, None))
        } else if token.openerp() {
            next!(token, tokens, {
                let outer_ty = if token.is_symbol() {
                    token
                } else {
                    return Err(ParserError::Token);
                };

                let inner_ty = read_type(tokens, input)?;
                handle_closer(tokens)?;
                Ok(Type::from_token(outer_ty, input, Some(inner_ty)))
            })
        } else {
            Err(ParserError::Token)
        }
    })
}

// Application
fn handle_application(t: Token, tokens: &mut Tokens, input: &str) -> Result<Ast> {
    let mut application = Vec::new();
    application.push(Ast::Identifier(get_symbol!(t, input)));
    while let Some(token) = tokens.next() {
        match token {
            t if t.closerp() => return Ok(Ast::Application(application)),
            t if t.openerp() => application.push(parse_paren_expr(tokens, input)?),
            t @ Token::Symbol(_) => application.push(Ast::Identifier(get_symbol!(t, input))),
            t @ Token::Integer(_) => {
                // TODO: should check that this integer can be represented at compile time
                let t = t.as_str(input).parse::<i32>().unwrap();
                application.push(Ast::Primitive(CompilePrimitive::Integer(t)));
            },
            t @ Token::String(_) => {
                let t = t.as_str(input).to_string();
                application.push(Ast::Primitive(CompilePrimitive::String(t)));
            }
            _ => todo!(),
        }
    }

    Err(ParserError::EOI)
}

fn handle_include(tokens: &mut Tokens, input: &str) -> Result<Ast> {
    let include = next!(t, tokens, {
        if t.is_symbol() {
            get_symbol!(t, input)
        } else {
            return Err(ParserError::Token);
        }
    });

    handle_closer(tokens)?;
    Ok(Ast::Include(include))
}

fn handle_closer(tokens: &mut Tokens) -> Result<()> {
    next!(t, tokens, {
        if !t.closerp() {
            Err(ParserError::Closer)
        } else {
            Ok(())
        }
    })
}

struct Tokens {
    tokens: Vec<Token>,
    position: usize,
}

impl Tokens {
    fn new(tokens: Vec<Token>) -> Self {
        Tokens {
            tokens: tokens,
            position: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
            Some(self.tokens[self.position-1])
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position])
        } else {
            None
        }
    }
}
