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
macro_rules! _typs_func {
    ($from:expr, $to:expr) => {
        Typ::func($from, $to)
    };

    ($from:expr, $to1:expr, $($tos:expr),+) => {
        func!($from, func!($to1, $($tos),+))
    }
}
pub use _typs_func as func;

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
    pub fn typ(&self, typ_ctx: &TypContext) -> TypResult<Typ> {
        match self {
            Term::Var { name } => match typ_ctx.get(name) {
                Some(t) => Ok(t.clone()),
                None => Err(TypError::Undefined(name.clone())),
            },
            Term::Abs {
                param_name,
                param_typ,
                body,
            } => {
                let typ_ctx = if param_name == "_" {
                    typ_ctx.clone()
                } else {
                    typ_ctx.insert(param_name.clone(), param_typ.clone())
                };

                let body_typ = body.typ(&typ_ctx)?;
                Ok(Typ::func(param_typ.clone(), body_typ.clone()))
            }
            Term::App { func, arg } => {
                let func_typ = func.typ(&typ_ctx)?;
                let arg_typ = arg.typ(&typ_ctx)?;

                match &func_typ {
                    Typ::Func { from, to } => {
                        let from = *from.clone();

                        if from == arg_typ {
                            Ok(*to.clone())
                        } else {
                            Err(TypError::Mismatch(from, arg_typ))
                        }
                    }
                    _ => Err(TypError::Expected("arrow type".to_string(), func_typ)),
                }
            }

            Term::Int(_) => Ok(Typ::atom("Int")),

            Term::If {
                cond,
                t_true,
                t_false,
            } => {
                let typ_cond = cond.typ(&typ_ctx)?;
                if typ_cond != Typ::atom("Bool") {
                    return Err(TypError::Mismatch(Typ::atom("Bool"), typ_cond));
                }

                let typ_true = t_true.typ(&typ_ctx)?;
                let typ_false = t_false.typ(&typ_ctx)?;

                if typ_true == typ_false {
                    Ok(typ_true)
                } else {
                    Err(TypError::Mismatch(typ_true, typ_false))
                }
            }
            Term::Seq(stmts) => {
                let mut typ_ctx = typ_ctx.clone();

                let mut typ_end = Typ::atom("Unit");
                for stmt in stmts {
                    typ_end = match stmt {
                        Stmt::Term(term) => term.typ(&typ_ctx)?,
                        Stmt::Let(var, term) => {
                            let t = term.typ(&typ_ctx)?;
                            typ_ctx = typ_ctx.insert(var.to_string(), t.clone());
                            t
                        }
                    };
                }

                Ok(typ_end)
            }
        }
    }
}
