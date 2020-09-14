use parser::Type;
use string_interner::Symbol;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct Environment {
    env: Rc<RefCell<_Environment>>,
}

impl Environment {
    /*
    pub fn new() -> Self {
        Environment {
            env: Rc::new(RefCell::new(_Environment::new())),
        }
    }
    */

    pub fn from_hashmap(map: HashMap<Symbol, Type>) -> Self {
        let env = _Environment {
            bindings: map,
            parent: None,
        };

        Environment {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn extend(&self) -> Self {
        let mut env = _Environment::new();
        env.parent = Some(self.clone());
        Environment {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn lookup_variable_type(&self, name: Symbol) -> Option<Type> {
        self.env.borrow().lookup_variable_type(name)
    }

    pub fn define_variable(&self, name: Symbol, ty: Type) {
        self.env.borrow_mut().define_variable(name, ty);
    }
}

#[derive(Default)]
pub struct _Environment {
    bindings: HashMap<Symbol, Type>,
    parent: Option<Environment>,
}

impl _Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn lookup_variable_type(&self, name: Symbol) -> Option<Type> {
        if let Some(ty) = self.bindings.get(&name) {
            Some(ty.clone())
        } else if let Some(ref env) = self.parent {
            env.lookup_variable_type(name)
        } else {
            None
        }
    }

    pub fn define_variable(&mut self, name: Symbol, ty: Type) {
        self.bindings.insert(name, ty);
    }
}
