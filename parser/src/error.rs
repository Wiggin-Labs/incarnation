use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParserError {
    EOI,
    Closer,
    Token,
    Item,
    Asm,
    ReturnType,
    NonfinalValue,
    Value,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParserError::EOI => write!(f, "Unexpected end of input"),
            ParserError::Closer => write!(f, "Expected closer"),
            ParserError::Token => write!(f, "Unexpected token"),
            ParserError::Item => write!(f, "Expected item"),
            ParserError::Asm => write!(f, "Inline assembly must be the only item in a procedure"),
            ParserError::ReturnType => write!(f, "Returned value does not match the expected return type"),
            ParserError::NonfinalValue => write!(f, "Primitive/identifier can only be the last item in a procedure"),
            ParserError::Value => write!(f, "Expected expression"),
        }
    }
}

