extern crate tokenizer;

mod error;

pub use error::ParserError;

use tokenizer::Token;

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum Ast {
    Include(Token),
    Define{
        name: Token,
        value: Box<Ast>
    },
    Primitive(CompilePrimitive),
    Asm(Vec<Token>),
    Application(Vec<Ast>),
    Identifier(Token),
    Lambda {
        args: Vec<Arg>,
        ret_ty: Option<Type>,
        body: Vec<Ast>,
    }
}

#[derive(Debug)]
pub struct Arg {
    name: Token,
    ty: Type,
}

impl Arg {
    pub fn new(name: Token, ty: Type) -> Self {
        Arg {
            name: name,
            ty: ty,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    U8,
    Usize,
    I32,
    Ptr(Box<Type>),
    /// ()
    Empty,
    /// !
    Never,
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
}

#[derive(Debug)]
pub enum CompilePrimitive {
    Integer(i32),
    String(String),
    Bool(bool),
}

pub fn parse(tokens: Vec<Token>, input: &str) -> Result<Vec<Ast>> {
    let mut ast = Vec::new();
    let mut tokens = Tokens::new(tokens);
    while tokens.peek().is_some() {
        if let Some(expr) = parse_expr(&mut tokens, input)? {
            ast.push(expr);
        } else {
            return Err(ParserError::Closer);
        }
    }
    /*
    while let Some(token) = tokens.next() {
        match token {
            Token::Comment(_) | Token::BlockComment(_) => {},
            t if t.openerp() => ast.push(parse_expr(&mut tokens, input)?),
            t if t.closerp() => return Err(ParserError::Closer),
            _ => return Err(ParserError::Token),
        }
    }
    */
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

fn parse_expr(tokens: &mut Tokens, input: &str) -> Result<Option<Ast>> {
    next!(token, tokens, {
        match token {
            Token::Comment(_) | Token::BlockComment(_) => parse_expr(tokens, input),
            t if t.closerp() => Ok(None),
            t if t.openerp() => Ok(Some(parse_paren_expr(tokens, input)?)),
            t @ Token::Symbol(_) => Ok(Some(Ast::Identifier(t))),
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
            t
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
            token
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
                token
            } else {
                return Err(ParserError::Token);
            }
        });

        let arg_type = read_type(tokens, input)?;

        // Read the argument closer
        handle_closer(tokens)?;

        args.push(Arg::new(arg_name, arg_type));
    };

    let ret_ty = if let Some(t) = tokens.peek() {
        if t.closerp() {
            // TODO: maybe this should be a `()` type?
            None
        } else {
            Some(read_type(tokens, input)?)
        }
    } else {
        return Err(ParserError::EOI);
    };

    // End of function preamble
    handle_closer(tokens)?;

    let mut body = Vec::new();
    while let Some(expr) = parse_expr(tokens, input)? {
        body.push(expr);
    }

    let func = Ast::Lambda {
        args: args,
        ret_ty: ret_ty,
        body: body,
    };
    Ok(Ast::Define{
        name: name,
        value: Box::new(func),
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
    application.push(Ast::Identifier(t));
    while let Some(token) = tokens.next() {
        match token {
            t if t.closerp() => return Ok(Ast::Application(application)),
            t if t.openerp() => application.push(parse_paren_expr(tokens, input)?),
            t @ Token::Symbol(_) => application.push(Ast::Identifier(t)),
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

fn handle_include(tokens: &mut Tokens, _input: &str) -> Result<Ast> {
    let include = next!(t, tokens, {
        if t.is_symbol() {
            t
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
