use std::path::PathBuf;

use anyhow::{Context as _, Result, anyhow};

use super::Faction;

pub fn parse(data: impl AsRef<str>) -> Result<Vec<Faction>> {
    data.as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .filter(|l| l.len() > 0) // strip empty lines
        .fold(vec![], |mut acc: Vec<Vec<String>>, line| {
            if line.starts_with("faction") {
                acc.push(vec![line.into()]);
            } else {
                let idx = acc.len() - 1;
                acc[idx].push(line.into());
            }
            acc
        })
        .into_iter()
        .map(|s| parse_faction(&s).with_context(|| format!("parsing line: {s:?}")))
        .collect()
}

fn parse_faction(lines: &[String]) -> Result<Faction> {
    let mut id = String::new();
    let mut name = String::new();
    let mut culture = String::new();
    let mut logo = PathBuf::new();
    for line in lines.iter() {
        let mut split = line.split(char::is_whitespace);
        let keyword = split
            .next()
            .ok_or_else(|| anyhow!("line didn't start with keyword"))
            .with_context(|| format!("parsing line {line}"))?;
        let value = split.remainder().map(|s| s.trim());
        if keyword == "faction" {
            id = value
                .ok_or_else(|| anyhow!("no faction name found"))
                .and_then(|s| {
                    s.split(',')
                        .nth(0)
                        .ok_or_else(|| anyhow!("no faction name found"))
                })
                .with_context(|| format!("parsing line {line}"))?
                .into();
            name = id.clone();
        } else if keyword == "culture" {
            culture = value
                .ok_or_else(|| anyhow!("line didn't have a value"))
                .with_context(|| format!("parsing line {line}"))?
                .into();
        } else if keyword == "loading_logo" {
            logo = value
                .ok_or_else(|| anyhow!("line didn't have a value"))
                .with_context(|| format!("parsing line {line}"))?
                .into();
        }
    }
    Ok(Faction {
        id,
        name,
        culture,
        logo,
    })
}
