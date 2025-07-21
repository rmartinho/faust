pub mod og {
    use logos::Logos;

    use crate::parse::utils::skip;

    #[derive(Logos, Clone, Debug, PartialEq)]
    #[logos()]
    pub enum Token<'source> {
        #[regex("[A-Za-z_][A-Za-z0-9_'+-]*", Lexer::slice, priority = 1)]
        Ident(&'source str),

        #[regex("[A-Za-z_][A-Za-z0-9_'+/.-]*", Lexer::slice, priority = 0)]
        Path(&'source str),

        #[token(",")]
        Comma,

        #[regex("\r?\n")]
        Newline,

        #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
        Int(u32),

        #[regex(";[^\n]*", skip)]
        Comment,

        #[regex("[ \t]+", skip)]
        Whitespace,
    }
}

pub mod rr {
    use logos::Logos;

    use crate::parse::utils::{skip, without_delimiters};

    #[derive(Logos, Clone, Debug, PartialEq)]
    #[logos()]
    pub enum Token<'source> {
        #[regex("\"[^\"]*\"", without_delimiters)]
        String(&'source str),

        #[token(":")]
        Colon,

        #[token(",")]
        Comma,

        #[token("-")]
        Minus,

        #[token("[")]
        OpenBracket,
        #[token("]")]
        CloseBracket,

        #[token("{")]
        OpenBrace,
        #[token("}")]
        CloseBrace,

        #[regex("true|false", |lex| if lex.slice() == "true" { true } else { false })]
        Bool(bool),

        #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
        Int(u32),

        #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
        Float(f64),

        #[regex(";[^\n]*", skip)]
        Comment,

        #[regex("[ \t\r\n]+", skip)]
        Whitespace,
    }
}
