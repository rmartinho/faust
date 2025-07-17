use std::path::PathBuf;

mod lexer;

mod parser {
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub og, "/parse/descr_sm_factions/parser.og.rs");
    lalrpop_mod!(pub rr, "/parse/descr_sm_factions/parser.rr.rs");
}

pub mod og {
    pub use super::lexer::og::Token;
    pub use super::parser::og::FactionsParser as Parser;
}

pub mod rr {
    pub use super::lexer::rr::Token;
    pub use super::parser::rr::FactionsParser as Parser;
}

#[derive(Debug)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub culture: String,
    pub logo: PathBuf,
}

impl TryFrom<Vec<Option<FactionProp>>> for Faction {
    type Error = ();

    fn try_from(value: Vec<Option<FactionProp>>) -> Result<Self, Self::Error> {
        let mut id = None;
        let mut name = None;
        let mut culture = None;
        let mut logo = None;
        for prop in value {
            let Some(prop) = prop else { continue };
            match prop {
                FactionProp::Id(it) => id = Some(it),
                FactionProp::Name(it) => name = Some(it),
                FactionProp::Culture(it) => culture = Some(it),
                FactionProp::Logo(it) => logo = Some(it),
            }
        }
        let (Some(id), Some(name), Some(culture), Some(logo)) = (id, name, culture, logo) else {
            return Err(());
        };
        Ok(Self {
            id,
            name,
            culture,
            logo,
        })
    }
}

enum FactionProp {
    Id(String),
    Name(String),
    Culture(String),
    Logo(PathBuf),
}
