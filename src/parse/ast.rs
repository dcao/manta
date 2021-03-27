pub type Symbol = &'static str;

#[derive(Debug)]
pub enum Expr<'a> {
    Assign(Symbol, &'a Expr<'a>),
    Monad(Builtin, &'a Expr<'a>),
    Dyad(Builtin, &'a Expr<'a>, &'a Expr<'a>),
    App(Symbol, &'a [&'a Expr<'a>]),
}

#[derive(Debug)]
pub enum Builtin {
    Plus,
    Minus,
    Times,
    Divide,
}
