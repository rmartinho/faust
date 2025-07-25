use std::path::PathBuf;

pub mod og;
pub mod rr;

#[derive(Debug)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub culture: String,
    pub logo: PathBuf,
}
