use logos::{Lexer, Logos, Skip, Source};

pub enum UpToTwo<T> {
    Zero,
    One(T),
    Two(T, T),
}

impl<T> UpToTwo<T> {
    pub fn push_to(self, mut v: Vec<T>) -> Vec<T> {
        match self {
            Self::Zero => {}
            Self::One(x) => v.push(x),
            Self::Two(x, y) => v.extend([x, y]),
        };
        v
    }

    pub fn to_vec(self) -> Vec<T> {
        self.push_to(vec![])
    }
}

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

pub fn spanned_lexer<'source, Tok: Logos<'source>>(
    source: &'source Tok::Source,
) -> impl Iterator<Item = Result<(usize, Tok, usize), Tok::Error>>
where
    Tok::Extras: Default,
{
    Tok::lexer(source)
        .spanned()
        .map(|(res, range)| res.map(|tok| (range.start, tok, range.end)))
}
