use std::{collections::HashMap, str::pattern::Pattern};

use anyhow::{Context as _, Result, anyhow};

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<Vec<Unit>> {
    data.as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .filter(|l| l.len() > 0) // strip empty lines
        .fold(vec![], |mut acc: Vec<Vec<String>>, line| {
            if line.starts_with("type") {
                acc.push(vec![line.into()]);
            } else {
                let idx = acc.len() - 1;
                acc[idx].push(line.into());
            }
            acc
        })
        .into_iter()
        .map(|s| parse_unit(&s).with_context(|| format!("parsing unit: {s:?}")))
        .collect()
}

fn parse_unit(lines: &[String]) -> Result<Unit> {
    let raw: Vec<_> = lines
        .iter()
        .map(|line| {
            let mut split = line.split(char::is_whitespace);
            let keyword = split
                .next()
                .ok_or_else(|| anyhow!("line didn't start with keyword"))
                .with_context(|| format!("parsing line {line}"))?;
            let value = split.remainder().map(|s| s.trim());
            Ok((keyword, value))
        })
        .collect::<Result<_>>()?;
    let entries: HashMap<_, _> = raw.iter().copied().collect();
    Ok(Unit {
        id: require_line_value(&entries, "type")?.into(),
        key: require_line_value(&entries, "dictionary")?.into(),
        category: require_line_value(&entries, "category")?.into(),
        class: require_line_value(&entries, "class")?.into(),
        ownership: parse_ownership(require_line_value(&entries, "ownership")?),
        stats: parse_statblock(&entries, &raw)?,
    })
}

fn parse_statblock(entries: &UnitEntries, raw: &[(&str, Option<&str>)]) -> Result<StatBlock> {
    let soldier_line = split_line(entries, "soldier", OPT_COMMA);
    let soldiers_line = split_line(entries, "soldiers", OPT_COMMA);
    let attribute_line = split_line(entries, "attributes", COMMA)?;
    let formations_line = split_line(entries, "formation", OPT_COMMA)?;
    let health_line = split_line(entries, "stat_health", OPT_COMMA)?;
    let pri_line = split_line(entries, "stat_pri", OPT_COMMA)?;
    let pri_attr_line = split_line(entries, "stat_pri_attr", OPT_COMMA)?;
    let sec_line = split_line(entries, "stat_sec", OPT_COMMA)?;
    let sec_attr_line = split_line(entries, "stat_sec_attr", OPT_COMMA)?;
    let pri_armour_line = split_line(entries, "stat_pri_armour", OPT_COMMA)?;
    let sec_armour_line = split_line(entries, "stat_sec_armour", OPT_COMMA)?;
    let heat_line = split_line(entries, "stat_heat", OPT_COMMA)?;
    let ground_line = split_line(entries, "stat_ground", OPT_COMMA)?;
    let mental_line = split_line(entries, "stat_mental", OPT_COMMA)?;
    let cost_line = split_line(entries, "stat_cost", OPT_COMMA)?;

    let speed_mod: f64 = require_line_value(entries, "move_speed_mod")
        .unwrap_or("1.0")
        .parse()?;

    Ok(StatBlock {
        soldiers: if let Ok(l) = soldier_line {
            l.get(1)
                .copied()
                .ok_or_else(|| anyhow!("missing # of soldiers"))
                .and_then(|s| Ok(s.parse()?))
        } else if let Ok(l) = soldiers_line {
            l.get(0)
                .copied()
                .ok_or_else(|| anyhow!("missing # of soldiers"))
                .and_then(|s| Ok(s.parse()?))
        } else {
            Err(anyhow!("missing soldier/soldiers info"))
        }
        .context("parsing # of soldiers")?,
        officers: raw.iter().filter(|(s, _)| *s == "officer").count() as _,
        mount: get_line_value(entries, "mount").map(Into::into),
        attributes: attribute_line
            .iter()
            .copied()
            .map(parse_attribute)
            .collect::<Result<_>>()
            .with_context(|| format!("parsing attributes from {attribute_line:?}"))?,
        speed_mod,
        formations: formations_line[5..]
            .into_iter()
            .copied()
            .map(|s| Ok(s.parse()?))
            .collect::<Result<_>>()
            .with_context(|| format!("parsing formations from {formations_line:?}"))?,
        hp: health_line
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing hit points"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing hit points from {health_line:?}"))?,
        hp_mount: health_line
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing mount hit points"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing mount hit points from {health_line:?}"))?,
        primary_weapon: parse_weapon(&pri_line, &pri_attr_line).with_context(|| {
            format!("parsing primary weapon from {pri_line:?}, {pri_attr_line:?}")
        })?,
        secondary_weapon: parse_weapon(&sec_line, &sec_attr_line).with_context(|| {
            format!("parsing secondary weapon from {sec_line:?}, {sec_attr_line:?}")
        })?,
        defense: parse_defense(&pri_armour_line)
            .with_context(|| format!("parsing defense from {pri_armour_line:?}"))?,
        defense_mount: parse_defense(&sec_armour_line)
            .with_context(|| format!("parsing mount defense from {sec_armour_line:?}"))?,
        heat: heat_line
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing heat bonus/penalty"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing heat bonus/penalty from {heat_line:?}"))?,
        ground_bonus: parse_ground(&ground_line)
            .with_context(|| format!("parsing ground bonuses from {ground_line:?}"))?,
        morale: mental_line
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing morale"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing morale from {mental_line:?}"))?,
        discipline: mental_line
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing morale"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing discipline from {mental_line:?}"))?,
        turns: cost_line
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing build turns"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing build turns from {cost_line:?}"))?,
        cost: cost_line
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing cost"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing cost from {cost_line:?}"))?,
        upkeep: cost_line
            .get(2)
            .copied()
            .ok_or_else(|| anyhow!("missing upkeep"))
            .and_then(|s| Ok(s.parse()?))
            .with_context(|| format!("parsing upkeep from {cost_line:?}"))?,
    })
}

fn parse_ground(strings: &[&str]) -> Result<GroundBonus> {
    Ok(GroundBonus {
        scrub: strings
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing scrub bonus"))
            .and_then(|s| Ok(s.parse()?))?,
        sand: strings
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing sand bonus"))
            .and_then(|s| Ok(s.parse()?))?,
        forest: strings
            .get(2)
            .copied()
            .ok_or_else(|| anyhow!("missing forest bonus"))
            .and_then(|s| Ok(s.parse()?))?,
        snow: strings
            .get(3)
            .copied()
            .ok_or_else(|| anyhow!("missing snow bonus"))
            .and_then(|s| Ok(s.parse()?))?,
    })
}

fn parse_weapon(stats: &[&str], attrs: &[&str]) -> Result<Weapon> {
    if let Some("no") = stats.get(0).copied() {
        return Ok(Weapon::default());
    }
    Ok(Weapon {
        factor: stats
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing weapon strength"))
            .and_then(|s| Ok(s.parse()?))?,
        charge: stats
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing charge bonus"))
            .and_then(|s| Ok(s.parse()?))?,
        missile: stats
            .get(2)
            .copied()
            .ok_or_else(|| anyhow!("missing missile"))
            .map(Into::into)?,
        range: stats
            .get(3)
            .copied()
            .ok_or_else(|| anyhow!("missing range"))
            .and_then(|s| Ok(s.parse()?))?,
        ammo: stats
            .get(4)
            .copied()
            .ok_or_else(|| anyhow!("missing ammo"))
            .and_then(|s| Ok(s.parse()?))?,
        lethality: stats
            .get(10)
            .copied()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.0),
        weapon_type: stats
            .get(5)
            .copied()
            .ok_or_else(|| anyhow!("missing weapon type"))
            .map(Into::into)?,
        tech_type: stats
            .get(6)
            .copied()
            .ok_or_else(|| anyhow!("missing tech type"))
            .map(Into::into)?,
        attributes: attrs
            .iter()
            .copied()
            .filter(|&a| a != "no")
            .map(parse_weapon_attribute)
            .collect::<Result<_>>()
            .context("parsing weapon attributes")?,
    })
}

fn parse_defense(strings: &[&str]) -> Result<Defense> {
    Ok(Defense {
        armor: strings
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing armor bonus"))
            .map(|s| s.parse().unwrap_or(0))?,
        skill: strings
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing skill bonus"))
            .map(|s| s.parse().unwrap_or(0))?,
        shield: strings
            .get(2)
            .copied()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
    })
}

fn parse_weapon_attribute(string: &str) -> Result<WeaponAttr> {
    Ok(match string {
        "ap" => WeaponAttr::ArmorPiercing,
        "bp" => WeaponAttr::BodyPiercing,
        "spear" => WeaponAttr::Spear,
        "long_pike" => WeaponAttr::LongPike,
        "short_pike" => WeaponAttr::ShortPike,
        "light_spear" => WeaponAttr::LightSpear,
        "prec" => WeaponAttr::Precharge,
        "thrown" => WeaponAttr::Thrown,
        "launching" => WeaponAttr::Launching,
        "area" => WeaponAttr::Area,
        "fire " => WeaponAttr::Fire,
        s if s.starts_with("spear_bonus_") => {
            WeaponAttr::SpearBonus(s["spear_bonus_".len()..].parse()?)
        }
        _ => WeaponAttr::Unknown,
    })
}

fn parse_attribute(string: &str) -> Result<Attr> {
    Ok(match string {
        "sea_faring" => Attr::SeaFaring,
        "hide_forest" => Attr::HideForest,
        "hide_improved_forest" => Attr::HideImprovedForest,
        "hide_long_grass" => Attr::HideLongGrass,
        "hide_anywhere" => Attr::HideAnywhere,
        "can_sap" => Attr::CanSap,
        "frighten_foot" => Attr::FrightenFoot,
        "frighten_mounted" => Attr::FrightenMounted,
        "can_run_amok" => Attr::CanRunAmok,
        "general_unit" => Attr::GeneralUnit,
        "cantabrian_circle" => Attr::CantabrianCircle,
        "no_custom" => Attr::NoCustom,
        "command" => Attr::Command,
        "screeching_women" => Attr::ScreechingWomen,
        "mercenary_unit" => Attr::MercenaryUnit,
        "hardy" => Attr::Hardy,
        "very_hardy" => Attr::VeryHardy,
        "extremely_hardy" => Attr::ExtremelyHardy,
        "inexhaustible" => Attr::Inexhaustible,
        "warcry" => Attr::Warcry,
        "druid" => Attr::Druid,
        "power_charge" => Attr::PowerCharge,
        "can_swim" => Attr::CanSwim,
        "is_peasant" => Attr::IsPeasant,
        "can_horde" => Attr::CanHorde,
        "legionary_name" => Attr::LegionaryName,
        "infinite_ammo" => Attr::InfiniteAmmo,
        "non_scaling" => Attr::NonScaling,
        "free_upkeep_unit" => Attr::FreeUpkeep,
        "can_withdraw" => Attr::CanWithdraw,
        "can_formed_charge" => Attr::FormedCharge,
        "knight" => Attr::Knight,
        "gunpowder_unit" => Attr::Gunpowder,
        "start_not_skirmishing" => Attr::UiOrAiHint,
        "stakes" => Attr::Stakes,
        "fire_by_rank" => Attr::FireByRank,
        "cannot_skirmish" => Attr::NoSkirmish,
        "unique_unit" => Attr::Unique,

        "guncavalry" | "crossbow" | "gunmen" | "peasant" | "pike" | "incendiary" | "artillery"
        | "cannon" | "rocket" | "mortar" | "explode" | "standard" | "wagon_fort" => {
            Attr::UiOrAiHint
        }
        s if s.starts_with("general_unit_upgrade") => {
            let mut split = s.split_whitespace();
            split.next(); // skip general_unit_upgrade
            let event = split
                .remainder()
                .map(|r| &r[1..r.len() - 1])
                .unwrap_or("marian_reforms");
            Attr::GeneralUnitUpgrade(event.into())
        }
        _ => Attr::Unknown,
    })
}

fn parse_ownership(owners: &str) -> Vec<String> {
    owners
        .split(OPT_COMMA)
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(Into::into)
        .collect()
}

type UnitEntries<'a> = HashMap<&'a str, Option<&'a str>>;

fn split_line<'a, P>(entries: &'a UnitEntries, key: &str, pat: P) -> Result<Vec<&'a str>>
where
    P: Pattern,
{
    Ok(require_line_value(entries, key)?
        .split(pat)
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect())
}

fn get_line_value<'a>(entries: &'a UnitEntries, key: &str) -> Option<&'a str> {
    entries.get(key).and_then(Option::as_deref)
}

fn require_line_value<'a>(entries: &'a UnitEntries, key: &str) -> Result<&'a str> {
    get_line_value(entries, key).ok_or_else(|| anyhow!("{key} not found"))
}

const OPT_COMMA: &[char] = &[',', ' '];
const COMMA: &str = ",";

#[derive(Debug)]
pub struct Unit {
    pub id: String,
    pub key: String,
    pub category: String,
    pub class: String,
    pub stats: StatBlock,
    pub ownership: Vec<String>,
}

#[derive(Debug)]
pub struct StatBlock {
    pub soldiers: u32,
    pub officers: u32,
    pub mount: Option<String>,
    pub attributes: Vec<Attr>,
    pub formations: Vec<Formation>,
    pub hp: i32,
    pub hp_mount: i32,
    pub primary_weapon: Weapon,
    pub secondary_weapon: Weapon,
    pub defense: Defense,
    pub defense_mount: Defense,
    pub heat: i32,
    pub ground_bonus: GroundBonus,
    pub morale: u32,
    pub discipline: Discipline,
    pub turns: u32,
    pub cost: u32,
    pub upkeep: u32,
    pub speed_mod: f64,
}

#[derive(Debug)]
pub struct Weapon {
    pub factor: u32,
    pub charge: u32,
    pub missile: String,
    pub range: u32,
    pub ammo: u32,
    pub lethality: f64,
    pub weapon_type: String,
    pub tech_type: String,
    pub attributes: Vec<WeaponAttr>,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            factor: Default::default(),
            charge: Default::default(),
            missile: Default::default(),
            range: Default::default(),
            ammo: Default::default(),
            lethality: Default::default(),
            weapon_type: "no".into(),
            tech_type: "no".into(),
            attributes: Default::default(),
        }
    }
}

pub type GroundBonus = silphium::model::GroundBonus;

pub type Defense = silphium::model::Defense;

#[derive(Debug, PartialEq, Eq)]
pub enum Attr {
    SeaFaring,
    HideForest,
    HideImprovedForest,
    HideLongGrass,
    HideAnywhere,
    CanSap,
    FrightenFoot,
    FrightenMounted,
    CanRunAmok,
    GeneralUnit,
    GeneralUnitUpgrade(String),
    CantabrianCircle,
    NoCustom,
    Command,
    ScreechingWomen,
    MercenaryUnit,
    Hardy,
    VeryHardy,
    ExtremelyHardy,
    Inexhaustible,
    Warcry,
    Druid,
    PowerCharge,
    CanSwim,
    IsPeasant,
    CanHorde,
    LegionaryName,
    InfiniteAmmo,
    NonScaling,

    FreeUpkeep,
    CanWithdraw,
    FormedCharge,
    Knight,
    Gunpowder,
    Stakes,
    FireByRank,
    NoSkirmish,
    Unique,

    UiOrAiHint,

    Unknown,
}

pub type Formation = silphium::model::Formation;
pub type Discipline = silphium::model::Discipline;

#[derive(Debug)]
pub enum WeaponAttr {
    ArmorPiercing,
    BodyPiercing,
    Spear,
    LongPike,
    ShortPike,
    LightSpear,
    Precharge,
    Thrown,
    Launching,
    Area,
    SpearBonus(u32),
    Fire,
    Unknown,
}
