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

pub fn type_check(ast: &[Ast]) -> Result<()> {
    // Add top level definitions to env right away
    // This avoids the C problem of values needing to be declared before their usage in a file.
    let mut bindings: HashMap<Symbol, Type> = HashMap::new();
    let mut holes = 0;
    for a in ast {
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
        'inner: for a in ast {
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
                    // TODO: we should probably allow If and Block in constants.
                    _ => (),
                }
            }
        }
    }

    let env = Environment::from_hashmap(bindings);

    for a in ast {
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

fn check_block(body: &[Ast], env: Environment) -> Result<Type> {
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
                let ty = check_application(a, env.clone())?;
                if i == body.len() - 1 {
                    return Ok(ty);
                }
            }
            Ast::Primitive(_) => return Ok(expr.ty()),
            Ast::Identifier(s) => if let Some(ident_ty) = env.lookup_variable_type(*s) {
                return Ok(ident_ty);
            } else {
                return Err(TypeError::UnboundIdentifier);
            },
            Ast::Block(b) => {
                let ty = check_block(b, env.extend())?;
                if i == body.len() - 1 {
                    return Ok(ty);
                }
            }
            Ast::If{ predicate, consequent, alternative } => {
                let ty = check_if(&**predicate, &**consequent, alternative, env.clone())?;
                if i == body.len() - 1 {
                    return Ok(ty.unwrap_or(Type::Empty));
                }
            }
        }
    }

    Ok(Type::Empty)
}

fn check_fun(ty: Type, body: &[Ast], env: Environment) -> Result<()> {
    let ret_ty = check_block(body, env.clone())?;
    if ty != Type::Empty && ty != Type::Never && ret_ty != ty {
        return Err(TypeError::Incompatible);
    }
    Ok(())
}

fn check_application(a: &[Ast], env: Environment) -> Result<Type> {
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
            Ast::Block(b) => if check_block(b, env.extend())? != arg_tys[j] {
                return Err(TypeError::Incompatible);
            },
            Ast::If{ predicate, consequent, alternative } => {
                if let Some(ty) = check_if(&**predicate, &**consequent, alternative, env.clone())? {
                    if arg_tys[j] != ty {
                        return Err(TypeError::Incompatible);
                    }
                } else {
                    return Err(TypeError::Incompatible);
                }
            },
            _ => unreachable!(),
        }
    }

    Ok(ret_ty)
}

macro_rules! match_if {
    ( $m:expr, $env:ident ) => {
        match $m {
            Ast::If { predicate, consequent, alternative } => {
                if let Some(ty) = check_if(&*predicate, &*consequent, alternative, $env.clone())? {
                    ty
                } else {
                    // TODO
                    Type::Empty
                    //return Err(TypeError::Incompatible);
                }
            },
            Ast::Block(b) => check_block(&b, $env.extend())?,
            Ast::Application(a) => check_application(&a, $env.clone())?,
            Ast::Identifier(s) => if let Some(ty) = $env.lookup_variable_type(*s) {
                ty
            } else {
                return Err(TypeError::UnboundIdentifier);
            },
            Ast::Primitive(_) => $m.ty(),
            _ => return Err(TypeError::Incompatible),
        }
    };
}

fn check_if(predicate: &Ast, consequent: &Ast, alternative: &Option<Box<Ast>>, env: Environment) -> Result<Option<Type>> {
    let pred_ty = match_if!(predicate, env);
    if pred_ty != Type::Bool {
        return Err(TypeError::Incompatible);
    }

    let cons_ty = match_if!(consequent, env);

    let alternative = if let Some(a) = alternative {
        a
    } else {
        return Ok(None);
    };

    let alt_ty = match_if!(&**alternative, env);

    if cons_ty != alt_ty {
        // TODO: not sure what to do here.
        return Ok(None);
        //return Err(TypeError::Incompatible);
    }
    Ok(Some(cons_ty))
}
