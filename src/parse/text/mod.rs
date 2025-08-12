use std::collections::HashMap;

use anyhow::Result;

use crate::parse::manifest::ParserMode;

mod bin;
mod txt;

pub fn parse_txt(data: impl AsRef<str>, _: ParserMode) -> Result<HashMap<String, String>> {
    txt::parse(data)
}

pub fn parse_bin(data: impl AsRef<[u8]>, _: ParserMode) -> Result<HashMap<String, String>> {
    bin::parse(data)
}
