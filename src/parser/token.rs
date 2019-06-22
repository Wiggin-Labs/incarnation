use string_interner::{DefaultStringInterner, Sym};

#[derive(Copy, Clone, Debug, PartialEq, is_enum_variant)]
pub enum Token {
    /// (
    LParen(Index),
    /// )
    RParen(Index),
    /// {
    LBrace(Index),
    /// }
    RBrace(Index),
    /// [
    LSBracket(Index),
    /// ]
    RSBracket(Index),
    /// <
    LABracket(Index),
    /// >
    RABracket(Index),
    Comment(Index),
    BlockComment(Index),
    /// # used for creating literals
    Pound(Index),
    String(Index),
    Integer(Index),
    Float(Index),
    Symbol(Index),
}

impl Token {
    pub fn index(&self) -> Index {
        use self::Token::*;
        match self {
            LParen(i) => *i,
            RParen(i) => *i,
            LBrace(i) => *i,
            RBrace(i) => *i,
            LSBracket(i) => *i,
            RSBracket(i) => *i,
            LABracket(i) => *i,
            RABracket(i) => *i,
            Comment(i) => *i,
            BlockComment(i) => *i,
            Pound(i) => *i,
            String(i) => *i,
            Integer(i) => *i,
            Float(i) => *i,
            Symbol(i) => *i,
        }
    }

    pub fn commentp(&self) -> bool {
        use self::Token::*;
        match self {
            Comment(_) | BlockComment(_) => true,
            _ => false,
        }
    }

    pub fn openerp(&self) -> bool {
        use self::Token::*;
        match self {
            LParen(_) | LBrace(_) | LSBracket(_) | LABracket(_) => true,
            _ => false,
        }
    }

    pub fn closerp(&self) -> bool {
        use self::Token::*;
        match self {
            RParen(_) | RBrace(_) | RSBracket(_) | RABracket(_) => true,
            _ => false,
        }
    }

    pub fn as_str<'a>(&self, input: &'a str) -> &'a str {
        let Index { start, end } = self.index();
        &input[start..end]
    }

    pub fn as_symbol(&self, input: &str, symbols: &mut DefaultStringInterner) -> Sym {
        let Index { start, end } = self.index();
        let symbol = &input[start..end];
        let mut s = symbol.chars();
        let mut buf = String::new();

        while let Some(c) = s.next() {
            match c {
                '\\' => match s.next() {
                    Some('n') => buf.push('\n'),
                    Some('t') => buf.push('\t'),
                    Some(c) => buf.push(c),
                    None => unreachable!(),
                },
                '|' => (),
                _ => buf.push(c),
            }
        }

        symbols.get_or_intern(buf)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Index {
    start: usize,
    end: usize,
}

impl Index {
    pub fn new(start: usize, end: usize) -> Self {
        Index {
            start: start - 1,
            end,
        }
    }
}
