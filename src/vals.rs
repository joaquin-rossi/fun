use crate::terms::Stmt;
use crate::terms::Term;

use std::rc::Rc;

// values

#[derive(Clone)]
pub enum Val {
    Abs {
        val_ctx: ValContext,
        param: String,
        body: Term,
    },
    Native(Rc<dyn Fn(Val) -> ValResult<Val>>),

    Bool(bool),
    Int(i32),
    Unit,
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Abs { val_ctx, param, body } => {
                write!(f, "Abs {{ val_ctx: {:?}, param: {:?}, body: {:?} }}", val_ctx, param, body)
            }
            Val::Native(_) => write!(f, "Native"),

            Val::Bool(b) => write!(f, "Bool({})", *b),
            Val::Int(i) => write!(f, "Int({})", *i),
            Val::Unit => write!(f, "Unit"),
        }
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Abs {
                val_ctx: _,
                param: _,
                body: _,
            }
            | Val::Native(_) => write!(f, "<fun>"),

            Val::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Val::Int(i) => write!(f, "{}", *i),
            Val::Unit => write!(f, "Unit"),
        }
    }
}

impl Val {
    pub fn native<F: Fn(Val) -> ValResult<Val> + 'static>(f: F) -> Val {
        Val::Native(Rc::new(f))
    }

    pub fn op1<F: Fn(Val) -> ValResult<Val> + 'static>(f: F) -> Val {
        Val::native(f)
    }

    pub fn op2<F: Fn(Val, Val) -> ValResult<Val> + Clone + 'static>(f: F) -> Val {
        Val::native(move |x| {
            let f = f.to_owned();
            Ok(Val::native(move |y| {
                let f = f.to_owned();
                let x = x.to_owned();
                f(x, y)
            }))
        })
    }

    pub fn op3<F: Fn(Val, Val, Val) -> ValResult<Val> + Clone + 'static>(f: F) -> Val {
        Val::native(move |x| {
            let f = f.to_owned();
            Ok(Val::native(move |y| {
                let f = f.to_owned();
                let x = x.to_owned();

                Ok(Val::native(move |z| {
                    let f = f.to_owned();
                    let x = x.to_owned();
                    let y = y.to_owned();

                    f(x, y, z)
                }))
            }))
        })
    }
}

// evaluation

pub type ValContext = crate::Map<String, Val>;

#[derive(Debug)]
pub enum ValError {}

impl std::fmt::Display for ValError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type ValResult<T> = Result<T, ValError>;

impl Term {
    // Term::eval() should only be called on terms known to pass type-checking
    pub fn eval(&self, val_ctx: &ValContext) -> ValResult<Val> {
        match self {
            Term::Var { name } => match val_ctx.get(name) {
                Some(v) => Ok(v.clone()),
                None => unreachable!(),
            },
            Term::Abs {
                param_name,
                param_typ: _,
                body,
            } => Ok(Val::Abs {
                val_ctx: val_ctx.clone(),
                param: param_name.clone(),
                body: *body.clone(),
            }),
            Term::App { func, arg } => {
                let func = func.eval(val_ctx)?;
                let arg = arg.eval(val_ctx)?;

                match &func {
                    Val::Abs { val_ctx, param, body } => {
                        let val_ctx = if param == "_" {
                            val_ctx.clone()
                        } else {
                            val_ctx.insert(param.clone(), arg)
                        };

                        body.eval(&val_ctx)
                    }
                    Val::Native(f) => f(arg),
                    _ => unreachable!(),
                }
            }

            Term::Int(i) => Ok(Val::Int(*i)),

            Term::If {
                cond,
                t_true,
                t_false,
            } => match cond.eval(val_ctx)? {
                Val::Bool(true) => t_true.eval(val_ctx),
                Val::Bool(false) => t_false.eval(val_ctx),
                _ => unreachable!(),
            },
            Term::Seq(stmts) => {
                let mut val_ctx = val_ctx.clone();

                let mut val_end = Val::Unit;
                for stmt in stmts {
                    val_end = match stmt {
                        Stmt::Term(term) => term.eval(&val_ctx)?,
                        Stmt::Let(var, term) => {
                            let v = term.eval(&val_ctx)?;
                            val_ctx = val_ctx.insert(var.to_string(), v.clone());
                            v
                        }
                    };
                }

                Ok(val_end)
            }
        }
    }
}
