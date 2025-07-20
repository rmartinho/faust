use std::{collections::HashMap, path::PathBuf};

use implicit_clone::unsync::IString;
use indexmap::IndexMap;
use logos::Logos as _;
use silphium::{ModuleMap, model::Module};
use tokio::{fs, io};

use crate::{
    args::Config,
    parse::{
        descr_mercenaries::Pool,
        descr_regions::Region,
        export_descr_buildings::{Building, Requires},
        manifest::ParserMode,
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

pub async fn parse_folder(cfg: &Config) -> io::Result<ModuleMap> {
    let root = &cfg.src_dir.join("data");
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
    let descr_sm_factions = if cfg.manifest.mode == ParserMode::Original {
        tokio::spawn(parse_descr_sm_factions_og(descr_sm_factions_path))
    } else {
        tokio::spawn(parse_descr_sm_factions_rr(descr_sm_factions_path))
    };
    let export_descr_unit = tokio::spawn(parse_export_descr_unit(export_descr_unit_path));
    let export_descr_buildings =
        tokio::spawn(parse_export_descr_buildings(export_descr_buildings_path));

    let mut text = expanded_bi.await??;
    text.extend(export_units.await??.into_iter());

    let pools = descr_mercenaries.await??;
    let regions = descr_regions.await??;
    let factions = descr_sm_factions.await??;
    let units = export_descr_unit.await??;
    let (require_aliases, buildings) = export_descr_buildings.await??;

    let model = build_model(
        units,
        factions,
        regions,
        pools,
        buildings,
        require_aliases,
        text,
    );

    let module_map = ModuleMap::from([(
        IString::from("rtw"),
        Module {
            id: "rtw".into(),
            name: "Vanilla".into(),
            banner: "faust/banner.png".into(),
            factions: model,
            aliases: Default::default(),
            eras: Default::default(),
        },
    )]);
    Ok(module_map)
}

fn build_model(
    raw_units: Vec<export_descr_unit::Unit>,
    raw_factions: Vec<descr_sm_factions::Faction>,
    _raw_regions: Vec<Region>,
    _raw_pools: Vec<Pool>,
    raw_buildings: Vec<Building>,
    aliases: HashMap<String, Requires>,
    text: HashMap<String, String>,
) -> IndexMap<IString, silphium::model::Faction> {
    let unit_map = raw_units.into_iter().map(|u| (u.id.clone(), u)).collect();
    let requires = build_requires(raw_buildings, &unit_map);

    let factions = raw_factions
        .into_iter()
        .map(|f| {
            (
                f.id.clone().into(),
                silphium::model::Faction {
                    id: f.id.clone().into(),
                    name: text.get(&f.name).cloned().unwrap_or(f.name.clone()).into(),
                    image: format!("{}", f.logo.display()).into(), // TODO
                    alias: None,                                   // TODO
                    eras: vec![].into(),                           // TODO
                    is_horde: false,                               // TODO
                    roster: unit_map
                        .values()
                        .filter(|u| {
                            available_to_faction(
                                requires.get(&u.id).unwrap_or(&Requires::False),
                                &f,
                                &aliases,
                            )
                        })
                        .map(|u| silphium::model::Unit {
                            id: u.id.clone().into(),
                            key: u.key.clone().into(),
                            name: text.get(&u.key).cloned().unwrap_or(u.key.clone()).into(),
                            image: format!("data/ui/units/{}/#{}.tga", f.id, u.key.to_lowercase()).into(), // TODO
                            soldiers: u.stats().soldiers,
                            officers: u.stats().officers,
                            attributes: vec![].into(), // TODO
                            formations: u.stats().formations.clone().into(), // TODO
                            hp: u.stats().hp,
                            hp_mount: u.stats().hp_mount,
                            primary_weapon: None,   // TODO
                            secondary_weapon: None, // TODO
                            defense: Some(u.stats().defense),
                            defense_mount: Some(u.stats().defense_mount),
                            heat: u.stats().heat,
                            ground_bonus: u.stats().ground_bonus,
                            morale: u.stats().morale,
                            discipline: u.stats().discipline,
                            turns: u.stats().turns,
                            cost: u.stats().cost,
                            upkeep: u.stats().upkeep,
                            eras: vec![].into(), // TODO
                        })
                        .collect(),
                },
            )
        })
        .collect();

    factions
}

fn is_general(unit: &export_descr_unit::Unit) -> bool {
    unit.rebalanced_stats
        .as_ref()
        .unwrap_or(&unit.stats)
        .attributes
        .contains(&export_descr_unit::Attr::GeneralUnit)
}

fn require_ownership(unit: &export_descr_unit::Unit) -> Requires {
    Requires::Factions(unit.ownership.clone())
}

fn build_requires(
    raw_buildings: Vec<Building>,
    unit_map: &HashMap<String, export_descr_unit::Unit>,
) -> HashMap<String, Requires> {
    raw_buildings
        .into_iter()
        .flat_map(|b| {
            b.caps.into_iter().map(move |r| {
                let owners = require_ownership(&unit_map[&r.unit]);
                (r.unit, Requires::And(vec![r.req, b.req.clone(), owners]))
            })
        })
        .chain(
            unit_map
                .values()
                .filter(|&u| is_general(u))
                .map(|u| (u.id.clone(), require_ownership(u))),
        ) // TODO general upgrades
        .fold(HashMap::new(), |mut h, (u, r)| {
            match h.entry(u).or_insert(Requires::Or(vec![])) {
                Requires::Or(items) => items.push(r),
                _ => unreachable!(),
            }
            h
        })
}

fn available_to_faction(
    req: &Requires,
    faction: &descr_sm_factions::Faction,
    aliases: &HashMap<String, Requires>,
) -> bool {
    evaluate(req, aliases, &Evaluator::faction(faction))
}

fn available_in_region(
    req: &Requires,
    region: &Region,
    aliases: &HashMap<String, Requires>,
) -> bool {
    evaluate(req, aliases, &Evaluator::region(region))
}

#[derive(Default)]
struct Evaluator {
    default: Option<bool>,
    faction: Option<EvaluatorChoices>,
    resource: Option<EvaluatorChoices>,
    hidden_resource: Option<EvaluatorChoices>,
    major_event: Option<EvaluatorChoices>,
}

impl Evaluator {
    fn faction(faction: &descr_sm_factions::Faction) -> Self {
        Self {
            faction: Some(EvaluatorChoices {
                map: [
                    (faction.id.clone(), true),
                    (faction.culture.clone(), true),
                    ("all".into(), true),
                ]
                .into(),
                default: Some(false),
            }),
            ..Default::default()
        }
    }

    fn region(region: &Region) -> Self {
        Self {
            hidden_resource: Some(EvaluatorChoices {
                map: region
                    .hidden_resources
                    .iter()
                    .map(|r| (r.clone(), true))
                    .collect(),
                default: Some(false),
            }),
            ..Default::default()
        }
    }
}

struct EvaluatorChoices {
    map: HashMap<String, bool>,
    default: Option<bool>,
}

impl EvaluatorChoices {
    fn get(&self, choice: &str) -> Option<bool> {
        self.map.get(choice).copied().or(self.default)
    }
}

fn evaluate(req: &Requires, aliases: &HashMap<String, Requires>, eval: &Evaluator) -> bool {
    do_evaluate(req, aliases, eval).unwrap_or(true)
}

fn do_evaluate(
    req: &Requires,
    aliases: &HashMap<String, Requires>,
    eval: &Evaluator,
) -> Option<bool> {
    match req {
        Requires::False => Some(false),
        Requires::Resource {
            id,
            factionwide: false,
        } => eval.resource.as_ref().and_then(|r| r.get(id)),
        Requires::HiddenResource {
            id,
            factionwide: false,
        } => eval.hidden_resource.as_ref().and_then(|r| r.get(id)),
        Requires::MajorEvent(event) => eval.major_event.as_ref().and_then(|r| r.get(event)),
        Requires::Factions(factions) => {
            let res = factions
                .iter()
                .map(|id| eval.faction.as_ref().and_then(|r| r.get(id)))
                .filter_map(|x| x)
                .collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res.iter().any(|x| *x))
            }
        }
        Requires::IsPlayer => Some(true),
        Requires::Alias(id) => do_evaluate(&aliases[id], aliases, eval),
        Requires::Not(requires) => do_evaluate(requires, aliases, eval).map(|r| !r),
        Requires::And(items) => {
            let res = items
                .iter()
                .map(|item| do_evaluate(item, aliases, eval))
                .filter_map(|x| x)
                .collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res.iter().all(|x| *x))
            }
        }
        Requires::Or(items) => {
            let res = items
                .iter()
                .map(|item| do_evaluate(item, aliases, eval))
                .filter_map(|x| x)
                .collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res.iter().any(|x| *x))
            }
        }
        _ => eval.default,
    }
}

async fn parse_text(path: PathBuf) -> io::Result<HashMap<String, String>> {
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

async fn parse_descr_sm_factions_og(path: PathBuf) -> io::Result<Vec<descr_sm_factions::Faction>> {
    let data = fs::read_to_string(&path).await?;
    let lex = descr_sm_factions::og::Token::lexer(&data);

    let factions = descr_sm_factions::og::Parser::new().parse(lex).unwrap();
    Ok(factions)
}

async fn parse_descr_sm_factions_rr(path: PathBuf) -> io::Result<Vec<descr_sm_factions::Faction>> {
    let data = fs::read_to_string(&path).await?;
    let lex = descr_sm_factions::rr::Token::lexer(&data);

    let factions = descr_sm_factions::rr::Parser::new().parse(lex).unwrap();
    Ok(factions)
}

async fn parse_export_descr_unit(path: PathBuf) -> io::Result<Vec<export_descr_unit::Unit>> {
    let data = fs::read_to_string(&path).await?;
    let lex = export_descr_unit::Token::lexer(&data);

    let units = export_descr_unit::Parser::new().parse(lex).unwrap();
    Ok(units)
}

async fn parse_export_descr_buildings(
    path: PathBuf,
) -> io::Result<(HashMap<String, Requires>, Vec<Building>)> {
    let data = fs::read_to_string(&path).await?;
    let lex = export_descr_buildings::Token::lexer(&data);

    let res = export_descr_buildings::Parser::new().parse(lex).unwrap();
    Ok(res)
}

const BOM: &str = "\u{feff}";
