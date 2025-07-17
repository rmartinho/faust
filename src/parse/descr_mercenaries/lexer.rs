use logos::Logos;

use crate::parse::utils::skip;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'source> {
    #[regex("[A-Za-z_][A-Za-z0-9_'+-]*", Lexer::slice)]
    Ident(&'source str),

    #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Int(u32),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Float(f64),

    #[token("-")]
    Hyphen,

    #[token(",")]
    Comma,

    #[regex("\r?\n")]
    Newline,

    #[regex(";[^\n]*", skip)]
    Comment,

    #[regex("[ \t]+", skip)]
    Whitespace,
}
