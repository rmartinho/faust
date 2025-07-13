use std::{io, path::PathBuf};

use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub id: IString,
    pub name: IString,
    #[serde(default)]
    pub mode: ParserMode,
    #[serde(default)]
    pub dir: Option<PathBuf>,
    #[serde(default = "default_campaign")]
    pub campaign: String,
    #[serde(default = "default_banner")]
    pub banner: PathBuf,
    // overrides
    // strings
}

fn default_campaign() -> String {
    "imperial_campaign".into()
}

fn default_banner() -> PathBuf {
    "faust/banner.png".into()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParserMode {
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "remastered")]
    Remastered,
}

impl Default for ParserMode {
    fn default() -> Self {
        Self::Remastered
    }
}

impl Manifest {
    pub fn from_yaml(r: impl io::Read) -> io::Result<Manifest> {
        serde_yml::from_reader(r).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
