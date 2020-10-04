use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    EndOfInput,
    Closer,
    Opener,
    Mode,
    Token,
    NoType,
    ConstantType,
    InvalidDisplacement
}

impl From<std::num::ParseIntError> for Error {
    fn from(_err: std::num::ParseIntError) -> Error {
        Error::InvalidDisplacement
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::EndOfInput => "Unexpected end of input",
            Error::Closer => "Unexpected closer",
            Error::Opener => "Unexpected opener",
            Error::Mode => "Invalid mode",
            Error::Token => "Unexpected token",
            Error::NoType => "Expected a type specifier",
            Error::ConstantType => "Given value does not match expected type",
            Error::InvalidDisplacement => "Invalid value given for displacement",
        })
    }
}

