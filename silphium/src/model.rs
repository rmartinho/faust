use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

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
    pub has_mount: bool,
    pub formations: IArray<Formation>,
    pub hp: u32,
    pub hp_mount: u32,
    pub primary_weapon: Option<Weapon>,
    pub secondary_weapon: Option<Weapon>,
    pub defense: Defense,
    pub defense_mount: Defense,
    pub heat: i32,
    pub ground_bonus: GroundBonus,
    pub morale: u32,
    pub discipline: Discipline,
    pub turns: u32,
    pub cost: u32,
    pub upkeep: u32,

    pub stamina: u32,
    pub inexhaustible: bool,
    pub infinite_ammo: bool,
    pub scaling: bool,
    pub abilities: IArray<Ability>,

    pub horde: bool,
    pub general: bool,
    pub mercenary: bool,
    pub legionary_name: bool,

    pub eras: IArray<IString>,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug,
)]
#[serde(rename_all = "snake_case")]
pub enum Ability {
    CantHide,
    HideImprovedForest,
    HideLongGrass,
    HideAnywhere,
    FrightenFoot,
    FrightenMounted,
    FrightenAll,
    CanRunAmok,
    // GeneralUnit,
    CantabrianCircle,
    Command,
    Warcry,
    PowerCharge,
    Chant,
    // Horde,
    // LegionaryName,
}

impl Display for Ability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CantHide => write!(f, "cant-hide"),
            Self::HideImprovedForest => write!(f, "hide-forest"),
            Self::HideLongGrass => write!(f, "hide-grass"),
            Self::HideAnywhere => write!(f, "hide-anywhere"),
            Self::FrightenFoot => write!(f, "frighten-foot"),
            Self::FrightenMounted => write!(f, "frighten-mounted"),
            Self::FrightenAll => write!(f, "frighten-all"),
            Self::CanRunAmok => write!(f, "can-run-amok"),
            Self::CantabrianCircle => write!(f, "cantabrian-circle"),
            Self::Command => write!(f, "command"),
            Self::Warcry => write!(f, "warcry"),
            Self::PowerCharge => write!(f, "power-charge"),
            Self::Chant => write!(f, "chant"),
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Weapon {
    #[serde(rename = "type")]
    pub class: WeaponType,
    pub factor: u32,
    pub is_missile: bool,
    pub charge: u32,
    pub range: u32,
    pub ammo: u32,
    pub lethality: f64,
    pub armor_piercing: bool,
    pub body_piercing: bool,
    pub pre_charge: bool,
    pub launching: bool,
    pub area: bool,
    pub fire: bool,
    pub spear_bonus: u32,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug,
)]
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

impl Defense {
    pub fn total(&self) -> u32 {
        self.armor + self.skill + self.shield
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug,
)]
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

impl Display for Formation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Square => write!(f, "square"),
            Self::Horde => write!(f, "horde"),
            Self::Phalanx => write!(f, "phalanx"),
            Self::Testudo => write!(f, "testudo"),
            Self::Wedge => write!(f, "wedge"),
            Self::Schiltrom => write!(f, "schiltrom"),
            Self::ShieldWall => write!(f, "shield_wall"),
        }
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug,
)]
#[serde(rename_all = "snake_case")]
pub enum Discipline {
    Low,
    Normal,
    Disciplined,
    Impetuous,
    Berserker,
}

impl Display for Discipline {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Normal => write!(f, "normal"),
            Self::Disciplined => write!(f, "disciplined"),
            Self::Impetuous => write!(f, "impetuous"),
            Self::Berserker => write!(f, "berserker"),
        }
    }
}
