use std::collections::HashMap;

use anyhow::Result;

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<HashMap<String, usize>> {
    Ok(data
        .as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .filter(|l| l.len() > 0) // strip empty lines
        .fold(
            (vec![], false),
            |(mut acc, mut collect): (Vec<String>, bool), line| {
                if line.starts_with("playable")
                    || line.starts_with("unlockable")
                    || line.starts_with("nonplayable")
                {
                    collect = true;
                } else if line.starts_with("end") {
                    collect = false;
                } else if collect {
                    acc.push(line.trim().into());
                }
                (acc, collect)
            },
        )
        .0
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect())
}
