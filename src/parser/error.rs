use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenizeError {
    EOF,
}

impl Display for TokenizeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenizeError::EOF => write!(f, "Unexpected end of input"),
        }
    }
}
