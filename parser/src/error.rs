use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParserError {
    EOI,
    Closer,
    Token,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParserError::EOI => write!(f, "Unexpected end of input"),
            ParserError::Closer => write!(f, "Expected closer"),
            ParserError::Token => write!(f, "Unexpected token"),
        }
    }
}

