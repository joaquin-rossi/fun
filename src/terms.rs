use crate::typs::Typ;

#[derive(Clone, Debug)]
pub enum Term {
    Var {
        name: String,
    },
    Abs {
        param_name: String,
        param_type: Typ,
        body: Box<Self>,
    },
    App {
        func: Box<Self>,
        arg: Box<Self>,
    },

    Int(i32),

    If {
        cond: Box<Self>,
        t_true: Box<Self>,
        t_false: Box<Self>,
    },
}

impl Term {
    pub fn var(name: &str) -> Self {
        Term::Var {
            name: name.to_string(),
        }
    }

    pub fn abs(param_name: &str, param_type: Typ, body: Self) -> Self {
        Term::Abs {
            param_name: param_name.to_string(),
            param_type,
            body: Box::new(body),
        }
    }

    pub fn app(func: Self, arg: Self) -> Self {
        Term::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    pub fn if_(cond: Self, t_true: Self, t_false: Self) -> Self {
        Term::If {
            cond: Box::new(cond),
            t_true: Box::new(t_true),
            t_false: Box::new(t_false),
        }
    }
}

#[macro_export]
macro_rules! _terms_var {
    ($name:expr) => {
        Term::var($name)
    };
}
pub use _terms_var as var;

#[macro_export]
macro_rules! _term_abs {
    ([$param:expr => $typ:expr], $body:expr) => {
        Term::abs($param, $typ, $body)
    };

    ([$param1:expr => $typ1:expr, $($params:expr => $typs:expr),+ ], $body:expr) => {
        abs!([$param1 => $typ1], abs!([ $($params => $typs),+ ], $body))
    };
}
pub use _term_abs as abs;

#[macro_export]
macro_rules! _term_app {
    ($fun:literal, $arg:literal) => {
        Term::app(var!($fun), var!($arg))
    };

    ($fun:literal, $arg:expr) => {
        Term::app(var!($fun), $arg)
    };

    ($fun:expr, $arg:literal) => {
        Term::app($fun, var!($arg))
    };

    ($fun:expr, $arg:expr) => {
        Term::app($fun, $arg)
    };

    ($fun:expr, $arg1:expr, $($args:expr),+) => {
        app!(app!($fun, $arg1), $($args),+)
    };
}
pub use _term_app as app;

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var { name, .. } => write!(f, "{}", name),
            Term::Abs {
                param_name,
                param_type,
                body,
            } => write!(f, "(fun {}:{} => {})", param_name, param_type, body),
            Term::App { func, arg } => write!(f, "({} {})", func, arg),

            Term::Int(i) => write!(f, "{}", *i),

            Term::If {
                cond,
                t_true,
                t_false,
            } => write!(f, "(if {} then {} else {})", cond, t_true, t_false),
        }
    }
}
