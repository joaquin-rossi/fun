use crate::terms::*;
use crate::typs::*;
use crate::vals::*;

pub mod ast;
pub mod terms;
pub mod typs;
pub mod vals;

pub(crate) type Map<K, V> = immutable_map::TreeMap<K, V>;

#[macro_export]
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("variant mismatch when casting to {}", stringify!($pat));
        }
    }};
}

// errors

#[derive(Debug)]
pub enum ProgramError {
    TypError(typs::TypError),
    ValError(vals::ValError),
}

pub type ProgramResult<T> = Result<T, ProgramError>;

// ctx

#[derive(Debug)]
pub struct ProgramContext {
    typ_ctx: TypContext,
    val_ctx: ValContext,
}

impl ProgramContext {
    pub fn get_val(&self, name: &str) -> Option<(Val, Typ)> {
        let val = self.val_ctx.get(name)?.clone();
        let typ = self.typ_ctx.get(name)?.clone();

        Some((val, typ))
    }

    // pub fn insert_typ(&self, name: &str, typ: Typ) -> Self {
    //     Context {
    //         t_ctx: self.t_ctx.
    //     }
    // }

    pub fn insert_term(&self, name: &str, term: &Term) -> ProgramResult<Self> {
        let (typ, val) = self.run(term)?;
        Ok(self.insert_val(name, &typ, &val))
    }

    pub fn insert_val(&self, name: &str, typ: &Typ, val: &Val) -> Self {
        ProgramContext {
            typ_ctx: self.typ_ctx.insert(name.to_string(), typ.clone()),
            val_ctx: self.val_ctx.insert(name.to_string(), val.clone()),
        }
    }

    pub fn typ(&self, term: &Term) -> TypResult<Typ> {
        term.typ(&self.typ_ctx)
    }

    pub fn eval(&self, term: &Term) -> ValResult<Val> {
        term.eval(&self.val_ctx)
    }

    pub fn run(&self, term: &Term) -> ProgramResult<(Typ, Val)> {
        let typ = match self.typ(term) {
            Ok(t) => Ok(t),
            Err(why) => Err(ProgramError::TypError(why)),
        }?;

        let val = match self.eval(term) {
            Ok(t) => Ok(t),
            Err(why) => Err(ProgramError::ValError(why)),
        }?;

        Ok((typ, val))
    }
}

impl ProgramContext {
    pub fn empty() -> Self {
        ProgramContext {
            typ_ctx: TypContext::new(),
            val_ctx: ValContext::new(),
        }
    }
}

impl Default for ProgramContext {
    fn default() -> Self {
        ProgramContext::empty()
            // unit
            .insert_val("Unit", &Typ::atom("Unit"), &Val::Unit)
            // bool
            .insert_val(
                "not",
                &typs::func!(Typ::atom("Bool"), Typ::atom("Bool")),
                &Val::op1(|x| {
                    let x = cast!(x, Val::Bool);

                    Ok(Val::Bool(!x))
                }),
            )
            .insert_val(
                "and",
                &typs::func!(Typ::atom("Bool"), Typ::atom("Bool"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Bool);
                    let y = cast!(y, Val::Bool);

                    Ok(Val::Bool(x && y))
                }),
            )
            .insert_val(
                "or",
                &typs::func!(Typ::atom("Bool"), Typ::atom("Bool"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Bool);
                    let y = cast!(y, Val::Bool);

                    Ok(Val::Bool(x || y))
                }),
            )
            .insert_val(
                "xor",
                &typs::func!(Typ::atom("Bool"), Typ::atom("Bool"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Bool);
                    let y = cast!(y, Val::Bool);

                    Ok(Val::Bool(x != y))
                }),
            )
            // int
            .insert_val(
                "neg",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int")),
                &Val::op1(|x| {
                    let x = cast!(x, Val::Int);

                    Ok(Val::Int(-x))
                }),
            )
            .insert_val(
                "add",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x + y))
                }),
            )
            .insert_val(
                "neg",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x + y))
                }),
            )
            .insert_val(
                "mul",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x * y))
                }),
            )
            .insert_val(
                "div",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x / y))
                }),
            )
            .insert_val(
                "mod",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x % y))
                }),
            )
            .insert_val(
                "eq",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Bool(x == y))
                }),
            )
            .insert_val(
                "gt",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Bool(x > y))
                }),
            )
            .insert_val(
                "gte",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Bool(x >= y))
                }),
            )
            .insert_val(
                "lt",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Bool(x < y))
                }),
            )
            .insert_val(
                "lte",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Bool")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Bool(x <= y))
                }),
            )
            // .insert_val(
            //     "~",
            //     &typs::func!(Typ::atom("Int"), Typ::atom("Int")),
            //     &Val::op1(|x| {
            //         let x = cast!(x, Val::Int);
            //
            //         Ok(Val::Int(!x))
            //     }),
            // )
            // .insert_val(
            //     "&",
            //     &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
            //     &Val::op2(|x, y| {
            //         let x = cast!(x, Val::Int);
            //         let y = cast!(y, Val::Int);
            //
            //         Ok(Val::Int(x & y))
            //     }),
            // )
            // .insert_val(
            //     "|",
            //     &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
            //     &Val::op2(|x, y| {
            //         let x = cast!(x, Val::Int);
            //         let y = cast!(y, Val::Int);
            //
            //         Ok(Val::Int(x | y))
            //     }),
            // )
            // .insert_val(
            //     "^",
            //     &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
            //     &Val::op2(|x, y| {
            //         let x = cast!(x, Val::Int);
            //         let y = cast!(y, Val::Int);
            //
            //         Ok(Val::Int(x ^ y))
            //     }),
            // )
            .insert_val(
                "shl",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x << y))
                }),
            )
            .insert_val(
                "shr",
                &typs::func!(Typ::atom("Int"), Typ::atom("Int"), Typ::atom("Int")),
                &Val::op2(|x, y| {
                    let x = cast!(x, Val::Int);
                    let y = cast!(y, Val::Int);

                    Ok(Val::Int(x >> y))
                }),
            )
    }
}
