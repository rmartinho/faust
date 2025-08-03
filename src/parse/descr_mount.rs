use std::collections::HashMap;

use anyhow::{Context as _, Result, anyhow};

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<HashMap<String, Mount>> {
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
        .map(|s| parse_mount(&s).with_context(|| format!("parsing mount: {s:?}")))
        .collect()
}

fn parse_mount(lines: &[String]) -> Result<(String, Mount)> {
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
    let id = require_line_value(&entries, "type")?;
    Ok((
        id.into(),
        Mount {
            id: id.into(),
            class: parse_class(require_line_value(&entries, "class")?),
            model: get_line_value(&entries, "model").map(Into::into),
            horse: get_line_value(&entries, "horse_type").map(Into::into),
        },
    ))
}

fn parse_class(string: &str) -> MountClass {
    match string {
        "horse" => MountClass::Horse,
        "camel" => MountClass::Camel,
        "elephant" => MountClass::Elephant,
        "chariot" => MountClass::Chariot,
        _ => MountClass::Unknown,
    }
}

type MountEntries<'a> = HashMap<&'a str, Option<&'a str>>;

fn get_line_value<'a>(entries: &'a MountEntries, key: &str) -> Option<&'a str> {
    entries.get(key).and_then(Option::as_deref)
}

fn require_line_value<'a>(entries: &'a MountEntries, key: &str) -> Result<&'a str> {
    get_line_value(entries, key).ok_or_else(|| anyhow!("{key} not found"))
}

#[derive(Debug)]
pub struct Mount {
    pub id: String,
    pub class: MountClass,
    pub model: Option<String>,
    pub horse: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MountClass {
    Horse,
    Camel,
    Elephant,
    Chariot,
    // ScorpionCart,
    Unknown,
}
