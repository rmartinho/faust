use std::collections::HashMap;

use anyhow::Context as _;
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};

use super::Faction;

pub fn parse(data: impl AsRef<str>) -> anyhow::Result<Vec<Faction>> {
    let mut filtered = data
        .as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .filter(|l| l.len() > 0) // strip empty lines
        .join("\n");
    let open_bracket_idx = filtered
        .find(OPEN_BRACKET)
        .context("replacing open bracket with open brace")?;
    let close_bracket_idx = filtered
        .rfind(CLOSE_BRACKET)
        .context("replacing close bracket with close brace")?;
    filtered.replace_range(open_bracket_idx..=open_bracket_idx, OPEN_BRACE);
    filtered.replace_range(close_bracket_idx..=close_bracket_idx, CLOSE_BRACE);

    let parsed: ProtoFactions = serde_json5::from_str(&format!("{{{filtered}}}"))
        .context("parsing JSON5-ified descr_sm_factions")?;

    parsed
        .factions
        .into_iter()
        .map(|(id, m)| {
            Ok(Faction {
                name: m
                    .get("string")
                    .and_then(|s| s.as_str())
                    .map(Into::into)
                    .ok_or_else(|| anyhow::Error::msg(format!("no `string` found for {id}")))?,
                culture: m
                    .get("culture")
                    .and_then(|s| s.as_str())
                    .map(Into::into)
                    .ok_or_else(|| anyhow::Error::msg(format!("no `culture` found for {id}")))?,
                logo: m
                    .get("logos")
                    .and_then(|s| s.as_object())
                    .and_then(|l| l.get("loading screen icon"))
                    .and_then(|s| s.as_str())
                    .map(Into::into)
                    .ok_or_else(|| {
                        anyhow::Error::msg(format!("no `loading screen icon` found for {id}"))
                    })?,
                    id
            })
        })
        .collect()
}

#[derive(Serialize, Deserialize)]
struct ProtoFactions {
    factions: HashMap<String, HashMap<String, serde_json::Value>>,
}

const OPEN_BRACKET: &str = "[";
const CLOSE_BRACKET: &str = "]";
const CLOSE_BRACE: &str = "}";
const OPEN_BRACE: &str = "{";
