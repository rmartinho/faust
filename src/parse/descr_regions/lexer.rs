use logos::Logos;

use crate::parse::utils::skip;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos()]
pub enum Token<'source> {
    #[regex("[A-Za-z_][A-Za-z0-9_'+-]*", Lexer::slice)]
    Ident(&'source str),

    #[token("\t")]
    Tab,

    #[regex("\r?\n")]
    Newline,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Int(u32),

    #[regex(";[^\n]*", skip)]
    Comment,

    #[regex(" +", skip)]
    Whitespace,
}
