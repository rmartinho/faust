use std::collections::HashMap;

use implicit_clone::ImplicitClone;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use yew::{AttrValue, Properties};

#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Module {
    pub id: AttrValue,
    pub name: AttrValue,
    pub logo: AttrValue,

    pub factions: IndexMap<AttrValue, Faction>,
    pub aliases: HashMap<AttrValue, AttrValue>,
}
#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Faction {
    pub id: AttrValue,
    pub name: AttrValue,
    pub image: AttrValue,
    #[serde(default)]
    pub alias: Option<AttrValue>,
}

impl Faction {
    pub fn id_or_alias(&self) -> AttrValue {
        if let Some(ref alias) = self.alias {
            return alias.clone();
        }
        return self.id.clone();
    }
}
