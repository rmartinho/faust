use logos::{Lexer, Logos, Skip, Source};

pub fn without_delimiters<'source, Token>(lex: &mut Lexer<'source, Token>) -> &'source str
where
    Token: Logos<'source>,
    for<'a> <Token as Logos<'source>>::Source: Source<Slice<'a> = &'a str>,
{
    let slice = lex.slice();
    &slice[1..slice.len() - 1]
}

pub fn skip<'source, Token>(_: &mut Lexer<'source, Token>) -> Skip
where
    Token: Logos<'source>,
{
    Skip
}

// #[derive(Logos, Debug, PartialEq)]
// #[logos(skip r"[ \t\n]+")]
// enum Token<'source> {
// 	#[regex("[A-Za-z][A-Za-z0-9_'+-]*", Lexer::slice)]
// 	Ident(&'source str),

// 	#[regex("[A-Za-z](?:[ A-Za-z0-9_'+-]*[A-Za-z0-9_'+-])?", Lexer::slice)]
// 	IdentSpace(&'source str),

// 	#[regex("[A-Za-z0-9_/+-]+", Lexer::slice)]
// 	Path(&'source str),

// 	#[regex("[0-9]+.[0-9]+", |lex| lex.slice().parse())]
// 	Float(f64),

// 	#[regex("[0-9]+", |lex| lex.slice().parse())]
// 	Int(u32),

//     #[regex("\"[^\"]*\"", |lex| &lex.slice()[1..lex.slice().len()-1])]
//     Quoted(&'source str),

//     #[regex(";[^\n]*", Lexer::slice)]
//     Comment(&'source str),
// }
