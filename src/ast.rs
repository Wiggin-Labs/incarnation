#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum Ast {
    Define(Tokens),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct Tokens {
    start: usize,
    end: usize,
}
