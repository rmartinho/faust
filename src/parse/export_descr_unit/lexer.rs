use logos::Logos;

use crate::parse::utils::{skip, without_delimiters};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos()]
pub enum Token<'source> {
    #[regex("[A-Za-z_][A-Za-z0-9_'+-]*", Lexer::slice)]
    Ident(&'source str),

    #[regex("\"[^\"]*\"", without_delimiters)]
    String(&'source str),

    #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Int(u32),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Float(f64),

    #[regex("\r?\n")]
    Newline,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,

    #[token(",")]
    Comma,

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    #[regex(";[^\n]*", skip)]
    Comment,

    #[regex("[ \t]+", skip)]
    Whitespace,
}
