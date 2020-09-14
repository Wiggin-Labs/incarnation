use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TypeError {
    UnboundIdentifier,
    Incompatible,
    Args,
}

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TypeError::UnboundIdentifier => write!(f, "Unbound identifier"),
            TypeError::Incompatible => write!(f, "Incompatible types"),
            TypeError::Args => write!(f, "Incorrect number of arguments"),
        }
    }
}
