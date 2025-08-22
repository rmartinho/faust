use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use anyhow::{Context as _, Result, anyhow, bail};
use pest::{Parser as _, iterators::Pairs};
use thiserror::Error;

use crate::{
    parse::manifest::ParserMode::{self, *},
    utils::parse_maybe_float_int,
};

mod requires {
    use pest_derive::Parser as PestParser;

    #[derive(PestParser)]
    #[grammar = "parse/requires.pest"]
    pub struct Parser;
}

pub fn parse(
    data: impl AsRef<str>,
    mode: ParserMode,
) -> Result<(HashMap<String, Requires>, Vec<Building>)> {
    let mut lines = data
        .as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .filter(|l| l.len() > 0); // strip empty lines

    let mut aliases = HashMap::new();
    if mode == Original || mode == Remastered {
        aliases.insert(
            "marian_reforms".into(),
            Requires::MajorEvent("marian_reforms".into()),
        );
    }
    let mut buildings = Vec::new();
    loop {
        match lines.next() {
            Some(line) if line.starts_with("alias") => {
                let (k, v) = parse_alias(&mut lines, line)
                    .with_context(|| format!("parsing alias {line}"))?;
                aliases.insert(k, v);
            }
            Some(line) if line.starts_with("levels") => {
                parse_building(&mut lines, line, &mut buildings, mode)
                    .with_context(|| format!("parsing building levels {line:?}"))?
            }
            Some(line) if line.starts_with("tags") => skip_block(&mut lines),
            Some(_) => {}
            None => break,
        }
    }

    Ok((aliases, buildings))
}

fn parse_alias<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    header: &'a str,
) -> Result<(String, Requires)> {
    let id = header
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow!("missing alias name"))?;

    let block = get_block(lines)?;
    let requires = block
        .into_iter()
        .find(|l| l.starts_with("requires"))
        .ok_or_else(|| anyhow!("missing requires line"))?;

    Ok((id.into(), parse_requires(requires)?))
}

fn parse_building<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    header: &'a str,
    buildings: &mut Vec<Building>,
    mode: ParserMode,
) -> Result<()> {
    let levels: Vec<_> = header.split_whitespace().skip(1).collect();

    let mut block = get_block(lines)?.into_iter();
    loop {
        match block.next() {
            Some(l)
                if l.split_whitespace()
                    .nth(0)
                    .map_or(false, |w| levels.contains(&w)) =>
            {
                let level = parse_level(&mut block, l, mode)
                    .with_context(|| format!("parsing level {l:?}"))?;
                buildings.push(level);
            }
            Some(l) => bail!("unexpected line in levels {l}"),
            None => break,
        }
    }
    Ok(())
}

fn parse_level<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    header: &'a str,
    mode: ParserMode,
) -> Result<Building> {
    let mut split = header.split_whitespace();
    split
        .next()
        .ok_or_else(|| anyhow!("invalid level header"))?;
    let mut req = split.remainder();
    if mode == Medieval2 && matches!(split.next(), Some("city") | Some("castle")) {
        req = split.remainder();
    }
    let req = req.map_or(Ok(Requires::None), parse_requires)?;
    let mut min = "village".into();
    let mut caps = Vec::new();
    let mut block = get_block(lines)?.into_iter();
    loop {
        match block.next() {
            Some("capability") => caps = parse_caps(&mut block, mode)?,
            Some(l) if l.starts_with("settlement_min") => {
                min = l
                    .split_whitespace()
                    .nth(1)
                    .map(Into::into)
                    .ok_or_else(|| anyhow!("missing settlement_min"))?
            }
            Some(_) => {}
            None => break,
        }
    }
    Ok(Building { req, caps, min })
}

fn parse_caps<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    mode: ParserMode,
) -> Result<Vec<RecruitOption>> {
    get_block(lines)?
        .into_iter()
        .filter(|l| l.starts_with("recruit"))
        .filter_map(|l| {
            Some(
                try {
                    let mut split = l.split_whitespace();
                    let kw = split
                        .next()
                        .ok_or_else(|| anyhow!("invalid recruit line"))?;
                    if kw != "recruit" && (mode != Medieval2 || kw != "recruit_pool") {
                        return None;
                    }
                    let mut split = split
                        .remainder()
                        .ok_or_else(|| anyhow!("missing unit"))?
                        .split('"');
                    let open_quote = split.next().ok_or_else(|| anyhow!("missing unit"))?;
                    if open_quote != "" {
                        Err(anyhow!("missing quotes around unit"))?;
                    }
                    let unit = split.next().ok_or_else(|| anyhow!("missing unit"))?;
                    let mut split = split
                        .remainder()
                        .ok_or_else(|| anyhow!("missing exp"))?
                        .trim()
                        .split_whitespace();
                    if kw == "recruit_pool" {
                        split
                            .next()
                            .ok_or_else(|| anyhow!("missing recruit pool data"))?;
                        split
                            .next()
                            .ok_or_else(|| anyhow!("missing recruit pool data"))?;
                        split
                            .next()
                            .ok_or_else(|| anyhow!("missing recruit pool data"))?;
                    }
                    let exp = split
                        .next()
                        .ok_or_else(|| anyhow!("missing exp"))
                        .and_then(parse_maybe_float_int)
                        .with_context(|| format!("parsing exp from {l}"))?;
                    let req = split
                        .remainder()
                        .map_or(Ok(Requires::None), parse_requires)?;
                    RecruitOption {
                        unit: unit.trim().into(),
                        exp,
                        req,
                    }
                },
            )
        })
        .collect()
}

fn parse_requires(requires: &str) -> Result<Requires> {
    use requires::*;
    let req = Parser::parse(Rule::Requires, requires)
        .with_context(|| format!("parsing requirement {requires}"))?
        .next()
        .unwrap();

    parse_req_op(req.into_inner())
}

fn parse_req_op(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    let pair = pairs.next().unwrap();
    Ok(match pair.as_rule() {
        Rule::Not => parse_req_not(pair.into_inner())?,
        Rule::ReqPrimary => parse_req_primary(pair.into_inner())?,
        Rule::Or => parse_req_or(pair.into_inner())?,
        Rule::And => parse_req_and(pair.into_inner())?,
        _ => bail!("unexpected parse result {pair}, expected `req_op`"),
    })
}

fn parse_req_or(pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    Ok(Requires::Or(
        pairs
            .map(|p| match p.as_rule() {
                Rule::Not => parse_req_not(p.into_inner()),
                Rule::ReqPrimary => parse_req_primary(p.into_inner()),
                Rule::Or => parse_req_or(p.into_inner()),
                Rule::And => parse_req_and(p.into_inner()),
                _ => bail!("unexpected parse result {p}, expected `req_or` child"),
            })
            .collect::<Result<_>>()?,
    ))
}

fn parse_req_and(pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    Ok(Requires::And(
        pairs
            .map(|p| match p.as_rule() {
                Rule::Not => parse_req_not(p.into_inner()),
                Rule::ReqPrimary => parse_req_primary(p.into_inner()),
                Rule::Or => parse_req_or(p.into_inner()),
                Rule::And => parse_req_and(p.into_inner()),
                _ => bail!("unexpected parse result {p}, expected `req_and` child"),
            })
            .collect::<Result<_>>()?,
    ))
}

fn parse_req_not(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    Ok(Requires::Not(Box::new(parse_req_primary(
        pairs.next().unwrap().into_inner(),
    )?)))
}
fn parse_req_primary(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    let pair = pairs.next().unwrap();
    Ok(match pair.as_rule() {
        Rule::Unknown => Requires::Unknown,
        Rule::Alias => Requires::Alias(pair.as_str().into()),
        Rule::Resource => parse_resource(pair.into_inner())?,
        Rule::HiddenResource => parse_hidden_resource(pair.into_inner())?,
        Rule::Event => Requires::MajorEvent(pair.as_str().into()),
        Rule::EventCount => parse_event_count(pair.into_inner())?,
        Rule::Factions => parse_factions(pair.into_inner())?,
        Rule::BuildingFactions => parse_building_factions(pair.into_inner())?,
        Rule::Toggle => Requires::IsToggled(pair.as_str().into()),
        Rule::Building => parse_building_present(pair.into_inner())?,
        Rule::BuildingLevel => parse_building_level(pair.into_inner())?,
        Rule::ReligionCond => parse_religion(pair.into_inner())?,
        Rule::NoTag => parse_no_tag(pair.into_inner())?,
        Rule::Port => Requires::Port,
        Rule::Player => Requires::IsPlayer,
        Rule::Diplomacy => parse_diplomacy(pair.into_inner())?,
        Rule::MajorityReligion => Requires::MajorityReligion(pair.as_str().into()),
        Rule::OfficialReligion => Requires::OfficialReligion,
        Rule::Capability => parse_capability(pair.into_inner())?,
        Rule::RegionReligion => parse_region_religion(pair.into_inner())?,
        _ => bail!("unexpected parse result {pair}, expected `req_primary`"),
    })
}

fn parse_resource(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let id = pairs.next().unwrap();
    let factionwide = match pairs.next() {
        Some(_) => true,
        None => false,
    };
    Ok(Requires::Resource {
        id: id.as_str().into(),
        factionwide,
    })
}

fn parse_hidden_resource(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let id = pairs.next().unwrap();
    let factionwide = match pairs.next() {
        Some(_) => true,
        None => false,
    };
    Ok(Requires::HiddenResource {
        id: id.as_str().into(),
        factionwide,
    })
}

fn parse_event_count(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let id = pairs.next().unwrap();
    let count = pairs.next().unwrap();
    Ok(Requires::EventCount {
        event: id.as_str().into(),
        count: parse_maybe_float_int(count.as_str())?,
    })
}

fn parse_factions(pairs: Pairs<requires::Rule>) -> Result<Requires> {
    Ok(Requires::Factions(
        pairs.map(|p| p.as_str().into()).collect(),
    ))
}

fn parse_building_factions(pairs: Pairs<requires::Rule>) -> Result<Requires> {
    Ok(Requires::BuildingFactions(
        pairs.map(|p| p.as_str().into()).collect(),
    ))
}

fn parse_building_present(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    let id = pairs.next().unwrap();

    let (queued, factionwide) = pairs.fold((false, false), |(q, f), p| match p.as_rule() {
        Rule::Queued => (true, f),
        Rule::Factionwide => (q, true),
        _ => (q, f),
    });
    Ok(Requires::BuildingPresent {
        id: id.as_str().into(),
        level: None,
        queued,
        factionwide,
    })
}

fn parse_building_level(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    let id = pairs.next().unwrap();
    let level = pairs.next().unwrap();

    let (queued, factionwide) = pairs.fold((false, false), |(q, f), p| match p.as_rule() {
        Rule::Queued => (true, f),
        Rule::Factionwide => (q, true),
        _ => (q, f),
    });
    Ok(Requires::BuildingPresent {
        id: id.as_str().into(),
        level: Some(level.as_str().into()),
        queued,
        factionwide,
    })
}

fn parse_religion(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let id = pairs.next().unwrap();
    let cmp = pairs.next().unwrap();
    let amount = pairs.next().unwrap();

    Ok(Requires::Religion {
        id: id.as_str().into(),
        cmp: cmp.as_str().parse()?,
        amount: parse_maybe_float_int(amount.as_str())?,
    })
}

fn parse_region_religion(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let id = pairs.next().unwrap();
    let amount = pairs.next().unwrap();

    Ok(Requires::Religion {
        id: id.as_str().into(),
        cmp: Cmp::Ge,
        amount: parse_maybe_float_int(amount.as_str())?,
    })
}

fn parse_diplomacy(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let status = pairs.next().unwrap();
    let faction = pairs.next().unwrap();
    Ok(Requires::Diplomacy {
        status: status.as_str().parse()?,
        faction: faction.as_str().into(),
    })
}

fn parse_capability(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    let id = pairs.next().unwrap();
    let amount = pairs.next().unwrap();
    Ok(Requires::Capability {
        capability: id.as_str().into(),
        amount: parse_maybe_float_int(amount.as_str())?,
    })
}

fn parse_no_tag(mut pairs: Pairs<requires::Rule>) -> Result<Requires> {
    use requires::Rule;
    let id = pairs.next().unwrap();

    let (queued, factionwide) = pairs.fold((false, false), |(q, f), p| match p.as_rule() {
        Rule::Queued => (true, f),
        Rule::Factionwide => (q, true),
        _ => (q, f),
    });

    Ok(Requires::NoBuildingTagged {
        tag: id.as_str().into(),
        queued,
        factionwide,
    })
}

fn skip_block<'a>(lines: &mut impl Iterator<Item = &'a str>) {
    let mut braces = 0;
    loop {
        match lines.next() {
            Some(line) => {
                braces += line.chars().filter(|c| *c == OPEN_BRACE).count();
                braces -= line.chars().filter(|c| *c == CLOSE_BRACE).count();
                if braces == 0 {
                    return;
                }
            }
            None => return,
        }
    }
}

fn get_block<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Vec<&'a str>> {
    let mut braces = 0;
    let mut block = Vec::new();
    loop {
        match lines.next() {
            Some("{") if braces == 0 => braces += 1,
            Some("}") if braces == 1 => return Ok(block),
            Some(line) => {
                braces += line.chars().filter(|c| *c == OPEN_BRACE).count();
                braces -= line.chars().filter(|c| *c == CLOSE_BRACE).count();
                block.push(line);
                if braces == 0 {
                    return Ok(block);
                }
            }
            None => bail!("invalid block"),
        }
    }
}

const OPEN_BRACE: char = '{';
const CLOSE_BRACE: char = '}';

#[derive(Debug)]
pub struct Building {
    pub req: Requires,
    pub caps: Vec<RecruitOption>,
    pub min: String,
}

#[derive(Debug)]
pub struct RecruitOption {
    pub unit: String,
    pub exp: u32,
    pub req: Requires,
}

#[derive(Debug, Default, Clone)]
pub enum Requires {
    #[default]
    None,
    False,
    Resource {
        id: String,
        factionwide: bool,
    },
    HiddenResource {
        id: String,
        factionwide: bool,
    },
    BuildingPresent {
        id: String,
        level: Option<String>,
        queued: bool,
        factionwide: bool,
    },
    MajorEvent(String),
    EventCount {
        event: String,
        count: u32,
    },
    Factions(Vec<String>),
    BuildingFactions(Vec<String>),
    Port,
    IsPlayer,
    IsToggled(String),
    Diplomacy {
        status: DipStatus,
        faction: String,
    },
    Capability {
        capability: String, // TODO list capabilities?
        amount: u32,
    },
    NoBuildingTagged {
        tag: String,
        queued: bool,
        factionwide: bool,
    },
    Religion {
        id: String,
        cmp: Cmp,
        amount: u32,
    },
    MajorityReligion(String),
    OfficialReligion,

    Alias(String),

    Unknown,

    Not(Box<Requires>),
    And(Vec<Requires>),
    Or(Vec<Requires>),
}

#[derive(Debug, Clone)]
pub enum Cmp {
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Error)]
pub struct CmpParseError;

impl Display for CmpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid comparation operator")
    }
}

impl FromStr for Cmp {
    type Err = CmpParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "<" => Ok(Cmp::Lt),
            "<=" => Ok(Cmp::Le),
            ">" => Ok(Cmp::Gt),
            ">=" => Ok(Cmp::Ge),
            _ => Err(CmpParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DipStatus {
    Allied,
    Protector,
    Protectorate,
    SameSuperfaction,
    AtWar,
}

#[derive(Debug, Error)]
pub struct DipStatusParseError;

impl Display for DipStatusParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid diplomacy status")
    }
}

impl FromStr for DipStatus {
    type Err = DipStatusParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "allied" => Ok(DipStatus::Allied),
            "protector" => Ok(DipStatus::Protector),
            "protectorate" => Ok(DipStatus::Protectorate),
            "same_superfaction" => Ok(DipStatus::SameSuperfaction),
            "at_war" => Ok(DipStatus::AtWar),
            _ => Err(DipStatusParseError),
        }
    }
}
