// use crate::repr::*;

pub type Symbol<'a> = &'a str;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Assign(Symbol<'a>, Box<Expr<'a>>),
    Monad(Builtin, Box<Expr<'a>>),
    Dyad(Builtin, Box<Expr<'a>>, Box<Expr<'a>>),
    App(Symbol<'a>, Box<[Expr<'a>]>),
    // TODO
    Array(i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Builtin {
    Plus,
    Minus,
    Times,
    Divide,
}
