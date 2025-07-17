use std::{collections::HashMap, env, path::PathBuf};

use logos::Logos as _;
use silphium::ModuleMap;
use tokio::{fs, io};

use crate::{
    args::Args,
    parse::{
        descr_mercenaries::Pool, descr_regions::Region, descr_sm_factions::Faction,
        export_descr_unit::Unit, manifest::ParserMode, text::TextMap,
        export_descr_buildings::{Requires, Building},
    },
};
pub use manifest::Manifest;

mod descr_mercenaries;
mod descr_regions;
mod descr_sm_factions;
mod export_descr_buildings;
mod export_descr_unit;
mod manifest;
mod text;
mod utils;

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
    let expanded_bi_path = root.join("text/expanded_bi.txt");
    let export_units_path = root.join("text/export_units.txt");
    let descr_mercenaries_path =
        root.join("world/maps/campaign/imperial_campaign/descr_mercenaries.txt"); // TODO cascading to base, etc
    let descr_regions_path = root.join("world/maps/base/descr_regions.txt"); // TODO cascading to base, etc
    let descr_sm_factions_path = root.join("descr_sm_factions.txt");
    let export_descr_unit_path = root.join("export_descr_unit.txt");
    let export_descr_buildings_path = root.join("export_descr_buildings.txt");

    let expanded_bi = tokio::spawn(parse_text(expanded_bi_path));
    let export_units = tokio::spawn(parse_text(export_units_path));
    let descr_mercenaries = tokio::spawn(parse_descr_mercenaries(descr_mercenaries_path));
    let descr_regions = tokio::spawn(parse_descr_regions(descr_regions_path));
    let descr_sm_factions = if manifest.mode == ParserMode::Original {
        tokio::spawn(parse_descr_sm_factions_og(descr_sm_factions_path))
    } else {
        tokio::spawn(parse_descr_sm_factions_rr(descr_sm_factions_path))
    };
    let export_descr_unit = tokio::spawn(parse_export_descr_unit(export_descr_unit_path));
    let export_descr_buildings = tokio::spawn(parse_export_descr_buildings(export_descr_buildings_path));

    let mut text = expanded_bi.await??;
    text.merge(export_units.await??);

    let pools = descr_mercenaries.await??;
    let regions = descr_regions.await??;
    let factions = descr_sm_factions.await??;
    let units = export_descr_unit.await??;
    let buildings = export_descr_buildings.await??;

    let _ = text;
    let _ = pools;
    let _ = regions;
    let _ = factions;
    let _ = units;
    let _ = buildings;

    println!("{:#?}", buildings);
    todo!()
}

async fn parse_text(path: PathBuf) -> io::Result<TextMap> {
    let buf = fs::read(&path).await?;
    let data = String::from_utf16le_lossy(&buf).replace(BOM, "");
    let lex = text::Token::lexer(&data);

    let map = text::Parser::new().parse(lex).unwrap();
    Ok(map)
}

async fn parse_descr_mercenaries(path: PathBuf) -> io::Result<Vec<Pool>> {
    let data = fs::read_to_string(&path).await?;
    let lex = descr_mercenaries::Token::lexer(&data);

    let pools = descr_mercenaries::Parser::new().parse(lex).unwrap();
    Ok(pools)
}

async fn parse_descr_regions(path: PathBuf) -> io::Result<Vec<Region>> {
    let data = fs::read_to_string(&path).await?;
    let lex = descr_regions::Token::lexer(&data);

    let pools = descr_regions::Parser::new().parse(lex).unwrap();
    Ok(pools)
}

async fn parse_descr_sm_factions_og(path: PathBuf) -> io::Result<Vec<Faction>> {
    let data = fs::read_to_string(&path).await?;
    let lex = descr_sm_factions::og::Token::lexer(&data);

    let factions = descr_sm_factions::og::Parser::new().parse(lex).unwrap();
    Ok(factions)
}

async fn parse_descr_sm_factions_rr(path: PathBuf) -> io::Result<Vec<Faction>> {
    let data = fs::read_to_string(&path).await?;
    let lex = descr_sm_factions::rr::Token::lexer(&data);

    let factions = descr_sm_factions::rr::Parser::new().parse(lex).unwrap();
    Ok(factions)
}

async fn parse_export_descr_unit(path: PathBuf) -> io::Result<Vec<Unit>> {
    let data = fs::read_to_string(&path).await?;
    let lex = export_descr_unit::Token::lexer(&data);

    let units = export_descr_unit::Parser::new().parse(lex).unwrap();
    Ok(units)
}

async fn parse_export_descr_buildings(path: PathBuf) -> io::Result<(HashMap<String, Requires>, Vec<Building>)> {
    let data = fs::read_to_string(&path).await?;
    let lex = export_descr_buildings::Token::lexer(&data);

    let res = export_descr_buildings::Parser::new().parse(lex).unwrap();
    Ok(res)
}

const BOM: &str = "\u{feff}";
