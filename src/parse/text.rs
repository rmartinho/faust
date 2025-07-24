use std::collections::HashMap;

pub fn parse(data: impl AsRef<str>) -> anyhow::Result<HashMap<String, String>> {
    use std::fmt::Write as _;

    data.as_ref()
        .lines() // split lines
        .filter_map(|l| l.split(is_comment_char).nth(0)) // strip comments
        .map(|l| l.trim()) // strip leading/trailing whitespace
        .filter(|l| l.len() > 0) // strip empty lines
        .fold(vec![], |mut acc: Vec<String>, line| {
            if line.starts_with(OPEN_BRACE) {
                acc.push(line.into());
            } else {
                let idx = acc.len() - 1;
                let _ = write!(&mut acc[idx], "\n{line}");
            }
            acc
        })
        .into_iter()
        .map(parse_tag)
        .collect()
}

fn is_comment_char(c: char) -> bool {
    c == NOT_SIGN || c == SEMICOLON
}

fn parse_tag(str: String) -> anyhow::Result<(String, String)> {
    let mut split = str.split(CLOSE_BRACE);
    let tag = split.next().ok_or(anyhow::Error::msg("failed parsing tag"))?;
    let value = split.remainder().ok_or(anyhow::Error::msg("failed parsing value"))?;
    Ok((tag.into(), value.into()))
}

const OPEN_BRACE: char = '{';
const CLOSE_BRACE: char = '}';
const NOT_SIGN: char = '¬';
const SEMICOLON: char = ';';
