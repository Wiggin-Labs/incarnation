#![feature(nll)]

#[macro_use]
extern crate derive_is_enum_variant;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;

pub mod ast;
pub mod parser;
