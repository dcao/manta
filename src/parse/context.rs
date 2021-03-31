// use super::ast::*;
// use bumpalo::Bump;
use lasso::{Rodeo, Spur};

pub struct Context {
    // arena: Bump,
    // TODO: use the interner only for symbols (e.g. `x)?
    // Regular var names can be represented as strings.
    interner: Rodeo<Spur>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            // arena: Bump::new(),
            interner: Rodeo::new(),
        }
    }

    // pub fn put_expr<'a>(&'a mut self, e: Expr<'a>) -> &'a Expr<'a> {
    //     self.arena.alloc(e)
    // }
}
