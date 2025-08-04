use std::{collections::HashMap, str::pattern::Pattern};

use anyhow::{Context as _, Result, anyhow};

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<HashMap<String, Model>> {
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
        .map(|s| parse_model(&s).with_context(|| format!("parsing model: {s:?}")))
        .collect()
}

fn parse_model(lines: &[String]) -> Result<(String, Model)> {
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
    let skeleton_line = split_line(&entries, "skeleton", ",")?;
    Ok((
        id.into(),
        Model {
            id: id.into(),
            skeleton: skeleton_line
                .get(0)
                .copied()
                .ok_or_else(|| anyhow!("missing skeleton for {id}"))?
                .into(),
        },
    ))
}

type ModelEntries<'a> = HashMap<&'a str, Option<&'a str>>;

fn split_line<'a, P>(entries: &'a ModelEntries, key: &str, pat: P) -> Result<Vec<&'a str>>
where
    P: Pattern,
{
    Ok(require_line_value(entries, key)?
        .split(pat)
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect())
}

fn get_line_value<'a>(entries: &'a ModelEntries, key: &str) -> Option<&'a str> {
    entries.get(key).and_then(Option::as_deref)
}

fn require_line_value<'a>(entries: &'a ModelEntries, key: &str) -> Result<&'a str> {
    get_line_value(entries, key).ok_or_else(|| anyhow!("{key} not found"))
}

#[derive(Debug)]
pub struct Model {
    pub id: String,
    pub skeleton: String,
}
