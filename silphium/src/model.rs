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
    pub banner: IString,

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
    #[serde(default)]
    pub is_horde: bool,
    pub roster: IArray<Unit>,
}

impl Faction {
    pub fn id_or_alias(&self) -> IString {
        if let Some(ref alias) = self.alias {
            return alias.clone();
        }
        return self.id.clone();
    }
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Era {
	pub id: IString,
    pub icon: IString,
    pub name: IString,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Unit {
	pub id: IString,
    pub name: IString,
    pub key: IString,
	pub image: IString,
    pub soldiers: u32,
    pub officers: u32,
    pub attributes: IArray<Attr>,
    pub formations: IArray<Formation>,
    pub hp: u32,
    pub hp_mount: u32,
    pub primary_weapon: Option<Weapon>,
    pub secondary_weapon: Option<Weapon>,
    pub defense: Option<Defense>,
    pub defense_mount: Option<Defense>,
    pub heat: i32,
    pub ground_bonus: GroundBonus,
    pub morale: u32,
    pub discipline: Discipline,
    pub turns: u32,
    pub cost: u32,
    pub upkeep: u32,
    pub eras: IArray<IString>,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Weapon {
    pub factor: u32,
    pub is_missile: bool,
    pub charge: u32,
    pub range: u32,
    pub ammo: u32,
    pub lethality: f64,
    pub attributes: IArray<WeaponAttr>,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WeaponType {
	Melee,
	Spear,
	Missile,
	Thrown,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
pub struct GroundBonus {
    pub scrub: i32,
    pub sand: i32,
    pub forest: i32,
    pub snow: i32,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
pub struct Defense {
    pub armor: u32,
    pub skill: u32,
    pub shield: u32,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Attr {
    NoHide,
    HideImprovedForest,
    HideLongGrass,
    HideAnywhere,
    FrightenFoot,
    FrightenMounted,
    CanRunAmok,
    GeneralUnit,
    CantabrianCircle,
    Command,
    Stamina(u32),
    Inexhaustible,
    Warcry,
    Chant,
    Horde,
    LegionaryName,
    InfiniteAmmo,
    NonScaling,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Formation {
    Square,
    Horde,
    Phalanx,
    Testudo,
    Wedge,
    Schiltrom,
    ShieldWall,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Discipline {
    Low,
    Normal,
    Disciplined,
    Impetuous,
    Berserker,
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WeaponAttr {
    ArmorPiercing,
    BodyPiercing,
    Precharge,
    Thrown,
    Launching,
    Area,
    SpearBonus(u32),
    Fire,
}
