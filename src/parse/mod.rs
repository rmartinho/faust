use std::{collections::HashMap, env, path::PathBuf};

use logos::Logos as _;
use silphium::ModuleMap;

mod text;
mod utils;

mod manifest;
pub use manifest::Manifest;
use tokio::{fs, io};

use crate::args::Args;

pub async fn parse_folder(args: &Args, manifest: Manifest) -> io::Result<ModuleMap> {
    let manifest_path = args
        .manifest
        .clone()
        .map(|m| m.parent().map(|p| p.to_path_buf()))
        .flatten()
        .unwrap_or_else(|| env::current_dir().unwrap());
    let root = manifest
        .dir
        .map(|d| manifest_path.join(d))
        .unwrap_or(manifest_path);
    let expanded_bi_txt = root.join("text/expanded_bi.txt");
    let export_units_txt = root.join("text/export_units.txt");

    let expanded_bi = tokio::spawn(parse_text_map(expanded_bi_txt));
    let export_units = tokio::spawn(parse_text_map(export_units_txt));

    let mut text = expanded_bi.await??;
    text.merge(export_units.await??);
    todo!()
}

async fn parse_text_map(path: PathBuf) -> io::Result<TextMap> {
    let buf = fs::read(&path).await?;
    let data = String::from_utf16le_lossy(&buf)
        .replace(CRLF, LF)
        .replace(BOM, "");
    let lex = text::Token::lexer(&data);

    let map = text::Parser::new().parse(lex).unwrap();
    Ok(TextMap { map })
}

struct TextMap {
    map: HashMap<String, String>,
}

impl TextMap {
    fn merge(&mut self, other: TextMap) {
        self.map.extend(other.map)
    }
}

// mod strings {
//     use pest_derive::Parser as Pest;

//     #[derive(Pest)]
//     #[grammar = "strings.pest"]
//     pub struct Parser;
// }

// mod descr_mercenaries {
//     use pest_derive::Parser as Pest;

//     #[derive(Pest)]
//     #[grammar = "descr_mercenaries.pest"]
//     pub struct Parser;
// }

// mod descr_strat {
//     use pest_derive::Parser as Pest;

//     #[derive(Pest)]
//     #[grammar = "descr_strat.pest"]
//     pub struct Parser;
// }

// mod export_descr_buildings {
//     use pest_derive::Parser as Pest;

//     #[derive(Pest)]
//     #[grammar = "export_descr_buildings.pest"]
//     pub struct Parser;
// }

// mod descr_sm_factions {
//     mod og {
//         use pest_derive::Parser as Pest;

//         #[derive(Pest)]
//         #[grammar = "descr_sm_factions.og.pest"]
//         pub struct Parser;
//     }
//     mod rr {
//         use pest_derive::Parser as Pest;

//         #[derive(Pest)]
//         #[grammar = "descr_sm_factions.rr.pest"]
//         pub struct Parser;
//     }
// }

const CRLF: &str = "\r\n";
const LF: &str = "\n";
const BOM: &str = "\u{feff}";
