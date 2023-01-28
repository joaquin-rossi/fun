use crate::terms::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Typ {
    Atom(String),
    Func { from: Box<Typ>, to: Box<Typ> },
}

impl Typ {
    pub fn atom(from: &str) -> Self {
        Typ::Atom(from.to_string())
    }

    pub fn func(from: Typ, to: Typ) -> Self {
        Typ::Func {
            from: Box::new(from),
            to: Box::new(to),
        }
    }
}

#[macro_export]
macro_rules! _types_func {
    ($from:expr, $to:expr) => {
        Typ::func($from, $to)
    };

    ($from:expr, $to1:expr, $($tos:expr),+) => {
        func!($from, func!($to1, $($tos),+))
    }
}
pub use _types_func as func;

impl std::fmt::Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Typ::Atom(s) => write!(f, "{}", s),
            Typ::Func { from, to } => write!(f, "({} -> {})", from, to),
        }
    }
}

// typing

pub type TypContext = crate::Map<String, Typ>;

#[derive(Clone, Debug)]
pub enum TypError {
    Undefined(String),
    Mismatch(Typ, Typ),
    Expected(String, Typ),
}

pub type TypResult<T> = Result<T, TypError>;

impl std::fmt::Display for TypError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypError::Undefined(var) => write!(f, "Variable \"{}\" isn't defined", var),
            TypError::Mismatch(exp, rec) => {
                write!(f, "Expected type \"{}\" but found \"{}\"", exp, rec)
            }
            TypError::Expected(exp, rec) => write!(f, "Expected {} but found \"{}\"", exp, rec),
        }
    }
}

impl Term {
    pub fn typ(&self, t_ctx: &TypContext) -> TypResult<Typ> {
        match self {
            Term::Var { name } => match t_ctx.get(name) {
                Some(t) => Ok(t.clone()),
                None => Err(TypError::Undefined(name.clone())),
            },
            Term::Abs {
                param_name,
                param_type,
                body,
            } => {
                let body_type = body.typ(&t_ctx.insert(param_name.clone(), param_type.clone()))?;
                Ok(Typ::func(param_type.clone(), body_type.clone()))
            }
            Term::App { func, arg } => {
                let func_type = func.typ(&t_ctx)?;
                let arg_type = arg.typ(&t_ctx)?;

                match &func_type {
                    Typ::Func { from, to } => {
                        let from = *from.clone();

                        if from == arg_type {
                            Ok(*to.clone())
                        } else {
                            Err(TypError::Mismatch(from, arg_type))
                        }
                    }
                    _ => Err(TypError::Expected("arrow type".to_string(), func_type)),
                }
            }

            Term::Int(_) => Ok(Typ::atom("Int")),

            Term::If {
                cond,
                t_true,
                t_false,
            } => {
                let typ_cond = cond.typ(&t_ctx)?;
                if typ_cond != Typ::atom("Bool") {
                    return Err(TypError::Mismatch(Typ::atom("Bool"), typ_cond));
                }

                let typ_true = t_true.typ(&t_ctx)?;
                let typ_false = t_false.typ(&t_ctx)?;

                if typ_true == typ_false {
                    Ok(typ_true)
                } else {
                    Err(TypError::Mismatch(typ_true, typ_false))
                }
            }
        }
    }
}
