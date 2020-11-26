#![feature(once_cell)]

use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::ops::Deref;
use std::sync::Mutex;

/// Global interner
pub static INTERNER: SyncLazy<Mutex<StringInterner>> = SyncLazy::new(|| Mutex::new(StringInterner::new()));

#[inline]
pub fn get_symbol(value: String) -> Symbol {
    INTERNER.lock().unwrap().get_symbol(value)
}

#[inline]
pub fn get_value(s: Symbol) -> Option<String> {
    INTERNER.lock().unwrap().get_value(s)
}

pub struct StringInterner {
    symbol_map: HashMap<String, usize>,
    symbols: Vec<String>,
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            symbol_map: HashMap::new(),
            symbols: Vec::new(),
        }
    }

    pub fn get_symbol(&mut self, value: String) -> Symbol {
        if self.symbol_map.contains_key(&value) {
            Symbol::new(*self.symbol_map.get(&value).unwrap())
        } else {
            self.symbol_map.insert(value.clone(), self.symbols.len());
            self.symbols.push(value);
            Symbol::new(self.symbols.len() - 1)
        }
    }

    pub fn get_value(&self, s: Symbol) -> Option<String> {
        if *s < self.symbols.len() {
            Some(self.symbols[*s].clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct Symbol(usize);

impl Symbol {
    pub fn new(v: usize) -> Self {
        Symbol(v)
    }
}

impl Deref for Symbol {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.0
    }
}
