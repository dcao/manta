use lasso::{Rodeo, Spur};

pub struct Context {
    // arena: Bumpalo<Expr>,
    interner: Rodeo<Spur>,
}
