use std::path::PathBuf;

use anyhow::Result;

use crate::parse::manifest::ParserMode;

mod og;
mod rr;

pub fn parse(data: impl AsRef<str>, mode: ParserMode) -> Result<Vec<Faction>> {
    match mode {
        ParserMode::Original | ParserMode::Medieval2 => og::parse(data),
        ParserMode::Remastered => rr::parse(data),
    }
}

#[derive(Debug)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub culture: String,
    pub logo: PathBuf,
}
