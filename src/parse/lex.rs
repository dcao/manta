use logos::Logos;

// TODO: comments
#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    // Basics
    #[regex("[a-zA-Z][a-zA-Z0-9]*")]
    Ident,
    #[regex(r"[0-9][0-9]*")]
    Integer,
    // TODO: handle exponent part etc.
    #[regex(r"[0-9][0-9]*\.[0-9]*")]
    Float,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("%")]
    Divide,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[token(";")]
    Semicolon,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    // #[regex(r"^/.*$", logos::skip)]
    Error,
}
