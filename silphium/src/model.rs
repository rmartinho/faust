use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use implicit_clone::{
    ImplicitClone,
    unsync::{IArray, IString},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;
use thiserror::Error;
use yew::Properties;

#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    IArray => #[serde(default, skip_serializing_if = "IArray::is_empty")],
    HashMap => #[serde(default, skip_serializing_if = "HashMap::is_empty")],
    IndexMap => #[serde(default, skip_serializing_if = "IndexMap::is_empty")],
)]
#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Module {
    #[serde(rename = "i")]
    pub id: IString,
    #[serde(rename = "n")]
    pub name: IString,
    #[serde(rename = "b")]
    pub banner: IString,

    #[serde(rename = "f")]
    pub factions: IndexMap<IString, Faction>,
    #[serde(rename = "r")]
    pub regions: IndexMap<IString, Region>,
    #[serde(rename = "p")]
    pub pools: IArray<Pool>,

    #[serde(rename = "a")]
    pub aliases: HashMap<IString, IString>,
    #[serde(rename = "e")]
    pub eras: IndexMap<IString, Era>,
}

#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    IArray => #[serde(default, skip_serializing_if = "IArray::is_empty")],
    bool => #[serde_as(as = "BoolFromInt")]
            #[serde(default, skip_serializing_if = "utils::is_false")],
)]
#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Faction {
    #[serde(rename = "i")]
    pub id: IString,
    #[serde(rename = "n")]
    pub name: IString,
    #[serde(rename = "b")]
    pub image: IString,
    #[serde(rename = "a")]
    pub alias: Option<IString>,
    #[serde(rename = "e")]
    pub eras: IArray<IString>,
    #[serde(rename = "h")]
    pub is_horde: bool,
    #[serde(rename = "r")]
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
    #[serde(rename = "i")]
    pub id: IString,
    #[serde(rename = "1")]
    pub icon: IString,
    #[serde(rename = "0")]
    pub icoff: IString,
    #[serde(rename = "n")]
    pub name: IString,
}

#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    IArray => #[serde(default, skip_serializing_if = "IArray::is_empty")],
    bool => #[serde_as(as = "BoolFromInt")]
            #[serde(default, skip_serializing_if = "utils::is_false")],
    u32 => #[serde(default, skip_serializing_if = "utils::is_zero_u32")],
)]
#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Unit {
    #[serde(rename = "i")]
    pub id: IString,
    #[serde(rename = "n")]
    pub name: IString,
    #[serde(rename = "k")]
    pub key: IString,
    #[serde(rename = "c")]
    pub class: UnitClass,
    #[serde(rename = "b")]
    pub image: IString,
    #[serde(rename = "#")]
    pub soldiers: u32,
    #[serde(rename = "o")]
    pub officers: u32,
    #[serde(rename = "M")]
    pub mount: MountType,
    #[serde(rename = "f")]
    #[serde_as(as = "OneOrMany<_>")]
    pub formations: IArray<Formation>,
    #[serde_with(skip_apply)]
    #[serde(rename = "h")]
    #[serde(default = "utils::one_u32", skip_serializing_if = "utils::is_one_u32")]
    pub hp: u32,
    #[serde(rename = "H")]
    pub hp_mount: u32,
    #[serde(rename = "w")]
    pub primary_weapon: Option<Weapon>,
    #[serde(rename = "W")]
    pub secondary_weapon: Option<Weapon>,
    #[serde(rename = "d")]
    pub defense: Defense,
    #[serde(rename = "D")]
    #[serde(default, skip_serializing_if = "utils::no_defense")]
    pub defense_mount: Defense,
    #[serde(rename = "t")]
    #[serde(default, skip_serializing_if = "utils::is_zero_i32")]
    pub heat: i32,
    #[serde(rename = "T")]
    #[serde(default, skip_serializing_if = "utils::no_ground_bonus")]
    pub ground_bonus: GroundBonus,
    #[serde(rename = "m")]
    pub morale: u32,
    #[serde(rename = "p")]
    pub discipline: Discipline,
    #[serde(rename = "r")]
    #[serde_with(skip_apply)]
    #[serde(default = "utils::one_u32", skip_serializing_if = "utils::is_one_u32")]
    pub turns: u32,
    #[serde(rename = "$")]
    pub cost: u32,
    #[serde(rename = "u")]
    pub upkeep: u32,

    #[serde(rename = "F")]
    pub stamina: u32,
    #[serde(rename = "E")]
    pub inexhaustible: bool,
    #[serde(rename = "A")]
    pub infinite_ammo: bool,
    #[serde_with(skip_apply)]
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "S")]
    #[serde(default = "utils::make_true", skip_serializing_if = "utils::is_true")]
    pub scaling: bool,
    #[serde(rename = "a")]
    #[serde_as(as = "OneOrMany<_>")]
    pub abilities: IArray<Ability>,

    #[serde(rename = "O")]
    pub horde: bool,
    #[serde(rename = "g")]
    pub general: bool,
    #[serde(rename = "R")]
    pub mercenary: bool,
    #[serde(rename = "N")]
    pub legionary_name: bool,

    // M2TW
    #[serde(rename = "0")]
    pub is_militia: bool,
    #[serde(rename = "*")]
    pub is_unique: bool,

    #[serde(rename = "e")]
    pub eras: IArray<IString>,
    #[serde(rename = "l")]
    pub tech_level: u32,

    #[serde(rename = "s")]
    pub move_speed: Option<u32>,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    ImplicitClone,
    Clone,
    Copy,
    Debug,
)]
#[repr(u8)]
pub enum UnitClass {
    Sword = 0,
    Spear = 1,
    Missile = 2,
    Cavalry = 3,
    General = 4,
    Animal = 5,
    Artillery = 6,
    Ship = 7,
}

impl UnitClass {
    pub fn all() -> [UnitClass; 8] {
        [
            UnitClass::Sword,
            UnitClass::Spear,
            UnitClass::Missile,
            UnitClass::Cavalry,
            UnitClass::General,
            UnitClass::Animal,
            UnitClass::Artillery,
            UnitClass::Ship,
        ]
    }
}

impl Display for UnitClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sword => write!(f, "sword"),
            Self::Spear => write!(f, "spear"),
            Self::Missile => write!(f, "missile"),
            Self::Cavalry => write!(f, "cavalry"),
            Self::General => write!(f, "general"),
            Self::Animal => write!(f, "animal"),
            Self::Artillery => write!(f, "artillery"),
            Self::Ship => write!(f, "ship"),
        }
    }
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    ImplicitClone,
    Clone,
    Copy,
    Debug,
)]
#[repr(u8)]
pub enum MountType {
    Foot = 0,
    Horse = 1,
    Other = 2,
}

impl MountType {
    pub fn has_mount(&self) -> bool {
        *self != MountType::Foot
    }

    pub fn has_mount_stats(&self) -> bool {
        *self == MountType::Other
    }
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    ImplicitClone,
    Clone,
    Copy,
    Debug,
)]
#[repr(u8)]
pub enum Ability {
    CantHide = 0,
    HideImprovedForest = 1,
    HideLongGrass = 2,
    HideAnywhere = 3,
    FrightenFoot = 4,
    FrightenMounted = 5,
    FrightenAll = 6,
    CanRunAmok = 7,
    CantabrianCircle = 8,
    Command = 9,
    Warcry = 10,
    PowerCharge = 11,
    Chant = 12,
    // LegionaryName = 13, // TODO
    FormedCharge = 14,
    Stakes = 15,
    Knight = 16,
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
            Self::FormedCharge => write!(f, "formed-charge"),
            Self::Stakes => write!(f, "stakes"),
            Self::Knight => write!(f, "knight"),
        }
    }
}

#[serde_with::apply(
    bool => #[serde(default, skip_serializing_if = "utils::is_false")],
    u32 => #[serde(default, skip_serializing_if = "utils::is_zero_u32")],
)]
#[derive(PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Weapon {
    #[serde(rename = "t")]
    pub class: WeaponType,
    #[serde(rename = "f")]
    pub factor: u32,
    #[serde(rename = "m")]
    pub is_missile: bool,
    #[serde(rename = "c")]
    pub charge: u32,
    #[serde(rename = "r")]
    pub range: u32,
    #[serde(rename = "#")]
    pub ammo: u32,
    #[serde(rename = "l")]
    #[serde(default = "utils::one_f64", skip_serializing_if = "utils::is_one_f64")]
    pub lethality: f64,
    #[serde(rename = "a")]
    pub armor_piercing: bool,
    #[serde(rename = "b")]
    pub body_piercing: bool,
    #[serde(rename = "P")]
    pub pre_charge: bool,
    #[serde(rename = "L")]
    pub launching: bool,
    #[serde(rename = "A")]
    pub area: bool,
    #[serde(rename = "F")]
    pub fire: bool,
    #[serde(rename = "s")]
    pub spear_bonus: u32,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    ImplicitClone,
    Clone,
    Copy,
    Debug,
)]
#[repr(u8)]
pub enum WeaponType {
    Melee = 0,
    Spear = 1,
    Missile = 2,
    Gunpowder = 3,
    Thrown = 4,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Melee => write!(f, "melee"),
            Self::Spear => write!(f, "spear"),
            Self::Missile => write!(f, "missile"),
            Self::Gunpowder => write!(f, "gunpowder"),
            Self::Thrown => write!(f, "thrown"),
        }
    }
}

#[derive(Default, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(into = "(i32, i32, i32, i32)", from = "(i32, i32, i32, i32)")]
pub struct GroundBonus {
    pub scrub: i32,
    pub sand: i32,
    pub forest: i32,
    pub snow: i32,
}

impl From<(i32, i32, i32, i32)> for GroundBonus {
    fn from(a: (i32, i32, i32, i32)) -> Self {
        Self {
            scrub: a.0,
            sand: a.1,
            forest: a.2,
            snow: a.3,
        }
    }
}

impl From<GroundBonus> for (i32, i32, i32, i32) {
    fn from(g: GroundBonus) -> Self {
        (g.scrub, g.sand, g.forest, g.snow)
    }
}

#[derive(Default, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Copy, Debug)]
#[serde(into = "(u32, u32, u32)", from = "(u32, u32, u32)")]
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

impl From<(u32, u32, u32)> for Defense {
    fn from(a: (u32, u32, u32)) -> Self {
        Self {
            armor: a.0,
            skill: a.1,
            shield: a.2,
        }
    }
}

impl From<Defense> for (u32, u32, u32) {
    fn from(d: Defense) -> Self {
        (d.armor, d.skill, d.shield)
    }
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    ImplicitClone,
    Clone,
    Copy,
    Debug,
)]
#[repr(u8)]
pub enum Formation {
    Horde = 0,
    Square = 1,
    Phalanx = 2,
    Testudo = 3,
    Wedge = 4,
    Schiltrom = 5,
    ShieldWall = 6,
}

#[derive(Debug, Error)]
pub struct FormationParseError;

impl Display for FormationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse formation")
    }
}

impl FromStr for Formation {
    type Err = FormationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "square" => Ok(Self::Square),
            "horde" => Ok(Self::Horde),
            "phalanx" => Ok(Self::Phalanx),
            "testudo" => Ok(Self::Testudo),
            "wedge" => Ok(Self::Wedge),
            "schiltrom" => Ok(Self::Schiltrom),
            "shield_wall" => Ok(Self::ShieldWall),
            _ => Err(FormationParseError),
        }
    }
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
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    ImplicitClone,
    Clone,
    Copy,
    Debug,
)]
#[repr(u8)]
pub enum Discipline {
    Low = 0,
    Normal = 1,
    Disciplined = 2,
    Impetuous = 3,
    Berserker = 4,
}

#[derive(Debug, Error)]
pub struct DisciplineParseError;

impl Display for DisciplineParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse discipline")
    }
}

impl FromStr for Discipline {
    type Err = DisciplineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "disciplined" => Ok(Self::Disciplined),
            "impetuous" => Ok(Self::Impetuous),
            "berserker" => Ok(Self::Berserker),
            _ => Err(DisciplineParseError),
        }
    }
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

#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    IArray => #[serde(default, skip_serializing_if = "IArray::is_empty")],
)]
#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Pool {
    #[serde(rename = "i")]
    pub id: IString,
    #[serde(rename = "r")]
    pub regions: IArray<IString>,
    #[serde(rename = "u")]
    pub units: IArray<PoolEntry>,
    #[serde(rename = "m")]
    pub map: IString,
}

#[serde_with::apply(
    IArray => #[serde(default, skip_serializing_if = "IArray::is_empty")],
    u32 => #[serde(default, skip_serializing_if = "utils::is_zero_u32")],
)]
#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct PoolEntry {
    #[serde(rename = "u")]
    pub unit: Unit,
    #[serde(rename = "x")]
    pub exp: u32,
    #[serde(rename = "r")]
    pub replenish: Replenish,
    #[serde(rename = "m")]
    pub max: u32,
    #[serde(rename = "s")]
    pub initial: u32,
    #[serde(rename = "R")]
    pub restrict: IArray<IString>,
}

#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    IArray => #[serde(default, skip_serializing_if = "IArray::is_empty")],
)]
#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
pub struct Region {
    #[serde(rename = "i")]
    pub id: IString,
    #[serde(rename = "l")]
    pub legion: Option<IString>,
    #[serde(rename = "c")]
    pub color: (u8, u8, u8),
    #[serde(rename = "r")]
    pub hidden_resources: IArray<IString>,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, ImplicitClone, Clone, Debug)]
#[serde(into = "(f64, f64)", from = "(f64, f64)")]
pub struct Replenish {
    pub min: f64,
    pub max: f64,
}

impl From<(f64, f64)> for Replenish {
    fn from(a: (f64, f64)) -> Self {
        Self { min: a.0, max: a.1 }
    }
}

impl From<Replenish> for (f64, f64) {
    fn from(r: Replenish) -> Self {
        (r.min, r.max)
    }
}

mod utils {
    pub fn make_true() -> bool {
        true
    }

    pub fn is_true(b: &bool) -> bool {
        *b
    }

    pub fn is_false(b: &bool) -> bool {
        !*b
    }

    pub fn is_zero_u32(u: &u32) -> bool {
        *u == 0
    }

    pub fn is_zero_i32(i: &i32) -> bool {
        *i == 0
    }

    pub fn is_one_u32(u: &u32) -> bool {
        *u == 1
    }

    pub fn is_one_f64(f: &f64) -> bool {
        *f == 1.0
    }

    pub fn one_u32() -> u32 {
        1
    }

    pub fn one_f64() -> f64 {
        1.0
    }

    pub fn no_defense(d: &super::Defense) -> bool {
        d.total() == 0
    }

    pub fn no_ground_bonus(g: &super::GroundBonus) -> bool {
        g.scrub == 0 && g.sand == 0 && g.forest == 0 && g.snow == 0
    }
}
