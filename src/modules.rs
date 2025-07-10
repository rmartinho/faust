use std::collections::HashMap;

use implicit_clone::{
    ImplicitClone,
    unsync::{IArray, IString},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Module {
    pub id: IString,
    pub name: IString,
    pub logo: IString,

    #[serde(default)]
    pub factions: IndexMap<IString, Faction>,
    #[serde(default)]
    pub aliases: HashMap<IString, IString>,
    #[serde(default)]
    pub eras: IndexMap<IString, Era>,
}
#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Faction {
    pub id: IString,
    pub name: IString,
    pub image: IString,
    #[serde(default)]
    pub alias: Option<IString>,
    #[serde(default)]
    pub eras: IArray<IString>,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Era {
    pub icon: IString,
    pub name: IString,
}

impl Faction {
    pub fn id_or_alias(&self) -> IString {
        if let Some(ref alias) = self.alias {
            return alias.clone();
        }
        return self.id.clone();
    }
}
