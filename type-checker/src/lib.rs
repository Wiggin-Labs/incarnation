extern crate parser;
extern crate string_interner;

mod env;
mod error;

pub use error::TypeError;

use env::Environment;

use parser::{Ast, Type};
use string_interner::Symbol;

use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, TypeError>;

pub fn type_check(ast: Vec<Ast>) -> Result<()> {
    // Add top level definitions to env right away
    // This avoids the C problem of values needing to be declared before their usage in a file.
    let mut bindings: HashMap<Symbol, Type> = HashMap::new();
    let mut holes = 0;
    for a in &ast {
        if let Ast::Define { name, ty, .. } = a {
            if *ty == Type::Hole {
                holes += 1;
            }
            bindings.insert(*name, ty.clone());
        } else if let Ast::Defn { name, ty, .. } = a {
            bindings.insert(*name, ty.clone());
        }
    }

    // We limit this loop to 20 iterations. Otherwise we might end in an infinite loop. I don't
    // think this restriction will be necessary with a more sophisticated type checker.
    let mut iterations = 0;
    while holes > 0 && iterations < 20 {
        iterations += 1;
        'inner: for a in &ast {
            if let Ast::Define { name, ty, value } = a {
                if *ty != Type::Hole {
                    continue 'inner;
                }

                match &**value {
                    Ast::Identifier(t) => {
                        let ty = if let Some(ty) = bindings.get(t) {
                            if *ty == Type::Hole {
                                continue 'inner;
                            }
                            ty.clone()
                        } else {
                            return Err(TypeError::UnboundIdentifier);
                        };
                        holes -= 1;
                        bindings.insert(*name, ty);
                    }
                    Ast::Application(v) => if let Ast::Identifier(t) = v[0] {
                        let ty = if let Some(ty) = bindings.get(&t) {
                            if *ty == Type::Hole {
                                continue 'inner;
                            }
                            ty.clone()
                        } else {
                            return Err(TypeError::UnboundIdentifier);
                        };
                        holes -= 1;
                        bindings.insert(*name, ty);
                    } else {
                        // TODO: error
                    },
                    _ => (),
                }
            }
        }
    }

    let env = Environment::from_hashmap(bindings);

    for a in &ast {
        match a {
            // TODO
            Ast::Include(_) => (),
            Ast::Define { name, .. } => assert!(env.lookup_variable_type(*name).unwrap() != Type::Hole),
            Ast::Defn { ty, args, body, .. } => {
                let fun_env = env.extend();
                for arg in args {
                    fun_env.define_variable(arg.name, arg.ty.clone());
                }
                check_fun(ty.arrow_split().1, body, fun_env)?;
            }
            // These should already be prevented by the parser
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn check_fun(ty: Type, body: &Vec<Ast>, env: Environment) -> Result<()> {
    for (i, expr) in body.iter().enumerate() {
        match expr {
            Ast::Include(_) | Ast::Asm(_) => (),
            Ast::Define { name, ty, value } => {
                match &**value {
                    Ast::Primitive(_) => if value.ty() != *ty {
                        return Err(TypeError::Incompatible);
                    },
                    Ast::Identifier(s) => if let Some(ident_ty) = env.lookup_variable_type(*s) {
                        if ident_ty != *ty {
                            return Err(TypeError::Incompatible);
                        }
                    } else {
                        return Err(TypeError::UnboundIdentifier);
                    },
                    Ast::Application(a) => if check_application(a, env.clone())? != *ty {
                        return Err(TypeError::Incompatible);
                    },
                    _ => unreachable!(),
                }
                env.define_variable(*name, ty.clone());
            }
            Ast::Defn { name, ty, args, body } => {
                env.define_variable(*name, ty.clone());
                let fun_env = env.extend();
                for arg in args {
                    fun_env.define_variable(arg.name, arg.ty.clone());
                }
                check_fun(ty.arrow_split().1, body, fun_env)?;
            }
            Ast::Application(a) => {
                let ret_ty = check_application(a, env.clone())?;
                // If this is the last expression in the procedure we make sure that it's type
                // matches the return type. If the return type is `()` we don't care.
                if i == body.len() - 1 && ty != Type::Empty {
                    if ret_ty != ty {
                        return Err(TypeError::Incompatible);
                    }
                }
            }
            Ast::Primitive(_) => if expr.ty() != ty {
                return Err(TypeError::Incompatible);
            },
            Ast::Identifier(s) => if let Some(ident_ty) = env.lookup_variable_type(*s) {
                if ident_ty != ty {
                    return Err(TypeError::Incompatible);
                }
            } else {
                return Err(TypeError::UnboundIdentifier);
            },
        }
    }
    Ok(())
}

fn check_application(a: &Vec<Ast>, env: Environment) -> Result<Type> {
    let app_ty = match a[0] {
        Ast::Identifier(s) => if let Some(ty) = env.lookup_variable_type(s) {
            ty
        } else {
            return Err(TypeError::UnboundIdentifier);
        },
        Ast::Application(_) => todo!(),
        // TODO
        _ => unreachable!(),
    };

    if !app_ty.is_arrow() {
        return Err(TypeError::Incompatible);
    }

    let (arg_tys, ret_ty) = app_ty.arrow_split();

    // Make sure that the number of arguments given is as expected
    if arg_tys.len() != a.len() -1 {
        return Err(TypeError::Args);
    }

    for j in 0..arg_tys.len() {
        match &a[j+1] {
            arg @ Ast::Primitive(_) => if arg.ty() != arg_tys[j] {
                return Err(TypeError::Incompatible);
            },
            Ast::Identifier(s) => if let Some(ty) = env.lookup_variable_type(*s) {
                if ty != arg_tys[j] {
                    return Err(TypeError::Incompatible);
                }
            } else {
                return Err(TypeError::UnboundIdentifier);
            },
            Ast::Application(a2) => if check_application(a2, env.clone())? != arg_tys[j] {
                return Err(TypeError::Incompatible);
            },
            _ => unreachable!(),
        }
    }

    Ok(ret_ty)
}
