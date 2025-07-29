use std::{collections::HashMap, io, path::PathBuf};

use anyhow::{Context as _, Result};
use implicit_clone::unsync::IString;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::parse::Evaluator;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default)]
    pub aliases: HashMap<IString, IString>,
    #[serde(default)]
    pub eras: IndexMap<IString, EraSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EraSpec {
    #[serde(default)]
    pub icon: Option<PathBuf>,
    #[serde(default)]
    pub icoff: Option<PathBuf>,
    #[serde(default)]
    pub name: Option<IString>,
    #[serde(flatten)]
    pub evaluator: Evaluator,
}

fn default_campaign() -> String {
    "imperial_campaign".into()
}

fn default_banner() -> PathBuf {
    "faust/banner.png".into()
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParserMode {
    Original,
    #[default]
    Remastered,
    Medieval2,
}

impl Manifest {
    pub fn from_yaml(r: impl io::Read) -> Result<Manifest> {
        Ok(serde_yml::from_reader(r).context("parsing manifest")?)
    }
}
