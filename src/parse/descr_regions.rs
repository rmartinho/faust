use anyhow::{Context as _, Result, anyhow};

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<str>, _: ParserMode) -> Result<Vec<Region>> {
    data.as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(';').nth(0)) // strip comments
        .filter(|l| l.len() > 0) // strip empty lines
        .fold(vec![], |mut acc: Vec<Vec<String>>, line| {
            if line.trim().len() > 0 {
                if !line.starts_with([' ', '\t']) {
                    acc.push(vec![line.trim().into()]);
                } else {
                    let idx = acc.len() - 1;
                    acc[idx].push(line.trim().into());
                }
            }
            acc
        })
        .into_iter()
        .map(|s| parse_region(&s).with_context(|| format!("parsing region: {s:?}")))
        .collect()
}

fn parse_region(lines: &[String]) -> Result<Region> {
    let legion = lines.iter().find(|l| l.starts_with("legion:"));
    let legion = legion
        .and_then(|l| l.split(':').remainder().map(str::trim))
        .map(Into::into);
    let lines: Vec<_> = lines
        .into_iter()
        .filter(|l| !l.starts_with("legion:"))
        .collect();

    let mut color_it = lines
        .get(4)
        .ok_or_else(|| anyhow!("missing color line"))?
        .split_whitespace()
        .map(|s| s.parse().context("parsing color"));
    Ok(Region {
        id: lines
            .get(0)
            .copied()
            .ok_or_else(|| anyhow!("missing id line"))?
            .clone(),
        legion,
        city: lines
            .get(1)
            .copied()
            .ok_or_else(|| anyhow!("missing city line"))?
            .clone(),
        color: (
            color_it.next().ok_or_else(|| anyhow!("missing red"))??,
            color_it.next().ok_or_else(|| anyhow!("missing green"))??,
            color_it.next().ok_or_else(|| anyhow!("missing blue"))??,
        ),
        hidden_resources: lines
            .get(5)
            .ok_or_else(|| anyhow!("missing hidden_resources line"))?
            .split(OPT_COMMA)
            .map(str::trim)
            .filter(|r| *r != "none")
            .map(Into::into)
            .collect(),
        // religions: todo!(),
    })
}

const OPT_COMMA: &[char] = &[',', ' '];
const COMMA: &str = ",";

#[derive(Debug)]
pub struct Region {
    pub id: String,
    pub legion: Option<String>,
    pub city: String,
    pub color: (u8, u8, u8),
    pub hidden_resources: Vec<String>,
    // pub religions: HashMap<String, u32>,
}
