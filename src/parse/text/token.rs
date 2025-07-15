use logos::Logos;

use crate::parse::utils::{skip, without_delimiters};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos()]
pub enum Token<'source> {
    //#[regex(r"\{[a-zA-Z0-9_-]+\}", without_delimiters, priority = 10)]
    //Key(&'source str),
    #[regex(r"\{[^}]+\}", without_delimiters, priority = 10)]
    Key(&'source str),

    #[regex("[^{ \t\n][^\n]*", Lexer::slice, priority = 10)]
    Value(&'source str),

    #[regex("¬[^\n]*", skip, priority = 20)]
    Comment,

    #[regex("[ \t\n]+", skip, priority = 20)]
    Whitespace,
}
