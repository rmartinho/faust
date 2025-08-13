use std::{collections::HashMap, str::pattern::Pattern};

use anyhow::{Context as _, Result, anyhow};

use crate::{parse::manifest::ParserMode, utils::parse_maybe_float_int};

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<Vec<Pool>> {
    data.as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .filter(|l| l.len() > 0) // strip empty lines
        .fold(vec![], |mut acc: Vec<Vec<String>>, line| {
            if line.starts_with("pool") {
                acc.push(vec![line.into()]);
            } else {
                let idx = acc.len() - 1;
                acc[idx].push(line.into());
            }
            acc
        })
        .into_iter()
        .map(|s| parse_pool(&s).with_context(|| format!("parsing pool: {s:?}")))
        .collect()
}

fn parse_pool(lines: &[String]) -> Result<Pool> {
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

    Ok(Pool {
        id: require_line_value(&entries, "pool")?.into(),
        regions: require_line_value(&entries, "regions")?
            .split(OPT_COMMA)
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(Into::into)
            .collect(),
        units: raw
            .into_iter()
            .filter(|(s, _)| *s == "unit")
            .map(|(_, s)| {
                parse_unit(s.ok_or_else(|| anyhow!("missing pool entry data"))?)
                    .with_context(|| format!("parsing pool entry: {s:?}"))
            })
            .collect::<Result<_>>()?,
    })
}

fn parse_unit(line: &str) -> Result<Unit> {
    let mut split = line.split(TAB_OR_COMMA);
    let id = split
        .next()
        .ok_or_else(|| anyhow!("missing unit id"))?
        .trim()
        .into();
    let rest = split
        .remainder()
        .ok_or_else(|| anyhow!("missing unit entry data"))?
        .trim();

    let data: Vec<_> = rest.split_whitespace().collect();

    Ok(Unit {
        id,
        exp: data
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing unit exp"))
            .and_then(parse_maybe_float_int)
            .with_context(|| format!("parsing experience from {line:?}"))?,
        cost: data
            .get(3)
            .copied()
            .ok_or_else(|| anyhow!("missing unit cost"))
            .and_then(parse_maybe_float_int)
            .with_context(|| format!("parsing cost from {line:?}"))?,
        replenish: (
            data.get(5)
                .copied()
                .ok_or_else(|| anyhow!("missing unit replenish lower bound"))
                .and_then(|s| Ok(s.parse()?))
                .with_context(|| format!("parsing unit replenish lower bound from {line:?}"))?,
            data.get(7)
                .copied()
                .ok_or_else(|| anyhow!("missing unit replenish upper bound"))
                .and_then(|s| Ok(s.parse()?))
                .with_context(|| format!("parsing unit replenish upper bound from {line:?}"))?,
        ),
        max: data
            .get(9)
            .copied()
            .ok_or_else(|| anyhow!("missing unit pool max"))
            .and_then(parse_maybe_float_int)
            .with_context(|| format!("parsing pool max from {line:?}"))?,
        initial: data
            .get(11)
            .copied()
            .ok_or_else(|| anyhow!("missing unit pool initial"))
            .and_then(|s| {
                if s == "end_year" {
                    Ok(0)
                } else {
                    parse_maybe_float_int(s)
                }
            })
            .with_context(|| format!("parsing pool initial from {line:?}"))?,
        restrict: if data.len() > 13 && data[12] == "restrict" {
            data[13..]
                .join(" ")
                .split(OPT_COMMA)
                .map(|s| s.trim())
                .filter(|s| s.len() > 0)
                .map(Into::into)
                .collect()
        } else {
            vec![]
        },
    })
}

type PoolEntries<'a> = HashMap<&'a str, Option<&'a str>>;

fn split_line<'a, P>(entries: &'a PoolEntries, key: &str, pat: P) -> Result<Vec<&'a str>>
where
    P: Pattern,
{
    Ok(require_line_value(entries, key)?
        .split(pat)
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect())
}

fn get_line<'a>(entries: &'a PoolEntries, key: &str) -> Option<Option<&'a str>> {
    entries.get(key).map(Option::as_deref)
}

fn get_line_value<'a>(entries: &'a PoolEntries, key: &str) -> Option<&'a str> {
    entries.get(key).and_then(Option::as_deref)
}

fn require_line<'a>(entries: &'a PoolEntries, key: &str) -> Result<Option<&'a str>> {
    get_line(entries, key).ok_or_else(|| anyhow!("{key} not found"))
}

fn require_line_value<'a>(entries: &'a PoolEntries, key: &str) -> Result<&'a str> {
    get_line_value(entries, key).ok_or_else(|| anyhow!("{key} not found"))
}

const OPT_COMMA: &[char] = &[',', ' '];
const TAB_OR_COMMA: &[char] = &[',', '\t'];

#[derive(Debug)]
pub struct Pool {
    pub id: String,
    pub regions: Vec<String>,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Unit {
    pub id: String,
    pub exp: u32,
    pub cost: u32,
    pub replenish: (f64, f64),
    pub max: u32,
    pub initial: u32,
    pub restrict: Vec<String>,
}
