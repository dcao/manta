use logos::{self, Lexer, Logos};
use std::iter;

// TODO: comments
#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    // Basics
    #[regex("[a-zA-Z][a-zA-Z0-9]*", |lex| lex.slice())]
    Ident(&'a str),
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice())]
    Text(&'a str),
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    // TODO: handle exponent part etc.
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

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
    #[token(r"\n")]
    Newline,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    #[regex(r"\s/.*\n", logos::skip)]
    Error,
}

struct Wrapper<'input> {
    lex: Lexer<'input, Token<'input>>,
}

impl<'input> Wrapper<'input> {
    fn new(lex: Lexer<'input, Token<'input>>) -> Wrapper<'input> {
        Wrapper { lex }
    }
}

pub type Spanned<Tok, Loc, Err> = Result<(Loc, Tok, Loc), Err>;

impl<'input> iter::Iterator for Wrapper<'input> {
    type Item = Spanned<Token<'input>, usize, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.lex.next()?;
        let span = self.lex.span();
        Some(Ok((span.start, tok, span.end)))
    }
}

pub fn tokens<'input>(
    source: &'input str,
) -> impl 'input + Iterator<Item = Spanned<Token, usize, ()>> {
    Wrapper::new(Token::lexer(source))
}
