use logos::Logos;

use crate::parse::utils::{skip, without_delimiters};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos()]
pub enum Token<'source> {
    #[regex("[A-Za-z_][A-Za-z0-9_'+-]*", Lexer::slice)]
    Ident(&'source str),

    #[regex("\"[^\"]*\"", without_delimiters)]
    String(&'source str),

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Int(u32),

    #[token(",")]
    Comma,

    #[token("<")]
    LessThan,
    #[token("<=")]
    LessOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterOrEqual,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,

    #[regex("\r?\n", skip)]
    Newline,

    #[regex(";[^\n]*", skip)]
    Comment,

    #[regex("[ \t]+", skip)]
    Whitespace,
}
