use logos::Logos;

use crate::parse::utils::{skip, without_delimiters};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos()]
pub enum Token<'source> {
    #[regex(r"\{[^}]+\}", without_delimiters)]
    Key(&'source str),

    #[regex("[^¬{ \t\r\n][^\r\n]*", Lexer::slice)]
    Value(&'source str),
    
    #[regex("¬[^\n]*", skip)]
    Comment,

    #[regex("[ \t\r\n]+", skip)]
    Whitespace,
}
