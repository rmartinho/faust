use anyhow::{Context as _, Result, anyhow};

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<String> {
    let line = data
        .as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .find(|l| l.starts_with("culture"))
        .ok_or_else(|| anyhow!("couldn't find default culture"))?;
    let mut split = line.split(char::is_whitespace);
    let _ = split
        .next()
        .ok_or_else(|| anyhow!("line didn't start with keyword"))
        .with_context(|| format!("parsing line {line}"))?;
    let value = split
        .remainder()
        .map(|s| s.trim())
        .ok_or_else(|| anyhow!("missing culture"))
        .with_context(|| format!("parsing line {line}"))?
        .to_string();
    Ok(value)
}
