use crate::terms::*;
use crate::typs::*;

#[derive(Debug)]
pub enum Decl {
    Type(String, Typ),
    Let(String, Term),
}

impl Decl {
    pub fn type_(name: &str, typ: Typ) -> Self {
        Decl::Type(name.to_string(), typ)
    }

    pub fn let_(name: &str, term: Term) -> Self {
        Decl::Let(name.to_string(), term)
    }
}
