#[macro_use]
extern crate derive_is_enum_variant;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;

mod error;
mod token;
mod tokenizer;

pub use self::error::TokenizeError;
pub use self::token::{Index, Token};
pub use self::tokenizer::Tokenizer;
