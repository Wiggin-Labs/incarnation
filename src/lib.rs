#![feature(nll)]
#![allow(ellipsis_inclusive_range_patterns)]

extern crate asm;
extern crate byteorder;
#[macro_use]
extern crate derive_is_enum_variant;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_interner;

pub mod ast;
pub mod parser;
pub mod executable;
