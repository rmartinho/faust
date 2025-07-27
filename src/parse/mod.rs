use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use implicit_clone::unsync::IString;
use indexmap::IndexMap;
use indicatif::{MultiProgress, ProgressBar};
use silphium::{
    ModuleMap,
    model::{Ability, Module, WeaponType},
};

use crate::{
    args::Config,
    parse::{
        descr_mercenaries::Pool,
        descr_regions::Region,
        export_descr_buildings::{Building, Requires},
        export_descr_unit::Attr,
        manifest::ParserMode,
    },
    utils::{LOOKING_GLASS, THINKING, path_fallback, progress_style, read_file},
};
pub use manifest::Manifest;

mod descr_mercenaries;
mod descr_regions;
mod descr_sm_factions;
mod export_descr_buildings;
mod export_descr_unit;
mod manifest;
mod text;

fn do_try_paths<'a>(root: &Path, paths: &[&'a str]) -> PathBuf {
    for path in paths.as_ref().iter() {
        let file = root.join(path);
        if file.exists() {
            return file;
        }
    }
    return paths.as_ref()[0].into();
}

fn try_paths<'a>(cfg: &Config, paths: impl AsRef<[&'a str]>) -> PathBuf {
    let first = do_try_paths(&cfg.src_dir, paths.as_ref());
    if first.exists() {
        first
    } else {
        do_try_paths(&cfg.fallback_dir, paths.as_ref())
    }
}

pub async fn parse_folder(cfg: &Config) -> Result<ModuleMap> {
    let expanded_path = path_fallback(cfg, "data/text/expanded.txt", false);
    let expanded_bi_path = path_fallback(cfg, "data/text/expanded_bi.txt", false);
    let export_units_path = path_fallback(cfg, "data/text/export_units.txt", false);
    let descr_mercenaries_path = try_paths(
        cfg,
        [
            &format!(
                "data/world/maps/campaign/{}/descr_mercenaries.txt",
                cfg.manifest.campaign
            ),
            "data/world/maps/base/descr_mercenaries.txt",
        ],
    );
    let descr_regions_path = try_paths(
        cfg,
        [
            &format!(
                "data/world/maps/campaign/{}/descr_regions.txt",
                cfg.manifest.campaign
            ),
            "data/world/maps/base/descr_regions.txt",
        ],
    );
    let descr_sm_factions_path = path_fallback(cfg, "data/descr_sm_factions.txt", false);
    let export_descr_unit_path = path_fallback(cfg, "data/export_descr_unit.txt", false);
    let export_descr_buildings_path = path_fallback(cfg, "data/export_descr_buildings.txt", false);

    let m = MultiProgress::new();

    let expanded_text_path = match cfg.manifest.mode {
        ParserMode::Original | ParserMode::Remastered => expanded_bi_path,
        ParserMode::Medieval2 => expanded_path,
    };
    let mut text = parse_progress(
        m.clone(),
        1,
        expanded_text_path.clone(),
        parse_text(expanded_text_path, cfg.manifest.mode),
    )
    .await?;
    let export_units = parse_progress(
        m.clone(),
        2,
        export_units_path.clone(),
        parse_text(export_units_path, cfg.manifest.mode),
    )
    .await?;
    text.extend(export_units.into_iter());

    let pools = parse_progress(
        m.clone(),
        3,
        descr_mercenaries_path.clone(),
        parse_descr_mercenaries(descr_mercenaries_path, cfg.manifest.mode),
    )
    .await?;
    let regions = parse_progress(
        m.clone(),
        4,
        descr_regions_path.clone(),
        parse_descr_regions(descr_regions_path, cfg.manifest.mode),
    )
    .await?;
    let factions = parse_progress(
        m.clone(),
        5,
        descr_sm_factions_path.clone(),
        parse_descr_sm_factions(descr_sm_factions_path, cfg.manifest.mode),
    )
    .await?;
    let units = parse_progress(
        m.clone(),
        6,
        export_descr_unit_path.clone(),
        parse_export_descr_unit(export_descr_unit_path, cfg.manifest.mode),
    )
    .await?;
    let (require_aliases, buildings) = parse_progress(
        m.clone(),
        7,
        export_descr_buildings_path.clone(),
        parse_export_descr_buildings(export_descr_buildings_path, cfg.manifest.mode),
    )
    .await?;

    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(progress_style());
    pb.set_prefix("[8/8]");
    pb.set_message(format!("{THINKING}processing mod data..."));
    pb.enable_steady_tick(Duration::from_millis(200));
    let model = build_model(
        cfg.manifest.mode,
        units,
        factions,
        regions,
        pools,
        buildings,
        require_aliases,
        text,
    );
    let _ = m.clear();

    let module_map = ModuleMap::from([(
        IString::from(&cfg.manifest.id),
        Module {
            id: cfg.manifest.id.clone().into(),
            name: cfg.manifest.name.clone().into(),
            banner: "faust/banner.png".into(),
            factions: model,
            aliases: Default::default(),
            eras: Default::default(),
        },
    )]);
    Ok(module_map)
}

fn parse_progress<'a, T>(
    m: MultiProgress,
    i: usize,
    path: PathBuf,
    fut: impl Future<Output = T> + 'a,
) -> impl Future<Output = T> + 'a {
    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(progress_style());
    pb.set_prefix(format!("[{}/7]", i));
    pb.set_message(format!("{LOOKING_GLASS}parsing {}...", path.display()));

    async move {
        let res = fut.await;
        pb.finish_with_message(format!(
            "{LOOKING_GLASS}parsing {}... done.",
            path.display()
        ));
        res
    }
}

fn build_model(
    mode: ParserMode,
    raw_units: Vec<export_descr_unit::Unit>,
    raw_factions: Vec<descr_sm_factions::Faction>,
    _raw_regions: Vec<Region>,
    _raw_pools: Vec<Pool>,
    raw_buildings: Vec<Building>,
    aliases: HashMap<String, Requires>,
    text: HashMap<String, String>,
) -> IndexMap<IString, silphium::model::Faction> {
    let unit_map: IndexMap<_, _> = raw_units.into_iter().map(|u| (u.id.clone(), u)).collect();
    let requires = build_requires(raw_buildings, &unit_map);

    let factions = raw_factions
        .into_iter()
        .map(|f| {
            (
                f.id.clone().into(),
                silphium::model::Faction {
                    id: f.id.clone().into(),
                    name: text
                        .get(&f.name.to_lowercase())
                        .cloned()
                        .unwrap_or(f.name.clone())
                        .into(),
                    image: format!(
                        "{}",
                        match mode {
                            ParserMode::Original | ParserMode::Medieval2 =>
                                PathBuf::from("data").join(&f.logo),
                            ParserMode::Remastered => f.logo.clone(),
                        }
                        .to_str()
                        .expect("invalid file name")
                        .to_lowercase()
                    )
                    .into(),
                    alias: None,         // TODO
                    eras: vec![].into(), // TODO
                    is_horde: false,     // TODO
                    roster: unit_map
                        .values()
                        .filter(|u| {
                            available_to_faction(
                                requires.get(&u.id).unwrap_or(&Requires::False),
                                &f,
                                &aliases,
                            )
                        })
                        .map(|u| {
                            let mut inexhaustible = false;
                            let mut stamina = 0;
                            let mut abilities = vec![];
                            let mut cant_hide = true;
                            let mut frighten_foot = false;
                            let mut frighten_mounted = false;
                            let mut infinite_ammo = false;
                            let mut non_scaling = false;
                            let mut horde = false;
                            let mut general = false;
                            let mut mercenary = false;
                            let mut legionary_name = false;
                            for attr in u.stats.attributes.iter() {
                                match attr {
                                    Attr::HideForest => cant_hide = false,
                                    Attr::HideImprovedForest => {
                                        cant_hide = false;
                                        abilities.push(Ability::HideImprovedForest)
                                    }
                                    Attr::HideLongGrass => {
                                        cant_hide = false;
                                        abilities.push(Ability::HideLongGrass)
                                    }
                                    Attr::HideAnywhere => {
                                        cant_hide = false;
                                        abilities.push(Ability::HideAnywhere)
                                    }
                                    Attr::FrightenFoot => frighten_foot = true,
                                    Attr::FrightenMounted => frighten_mounted = true,
                                    Attr::CanRunAmok => abilities.push(Ability::CanRunAmok),
                                    Attr::GeneralUnit => general = true,
                                    // Attr::GeneralUnitUpgrade(_) => todo!(),
                                    Attr::CantabrianCircle => {
                                        abilities.push(Ability::CantabrianCircle)
                                    }
                                    Attr::Command => abilities.push(Ability::Command),
                                    Attr::Druid | Attr::ScreechingWomen => {
                                        abilities.push(Ability::Chant)
                                    }
                                    Attr::MercenaryUnit => mercenary = true,
                                    Attr::Hardy => stamina += 2,
                                    Attr::VeryHardy => stamina += 4,
                                    Attr::ExtremelyHardy => stamina += 8,
                                    Attr::Inexhaustible => inexhaustible = true,
                                    Attr::Warcry => abilities.push(Ability::Warcry),
                                    Attr::PowerCharge => abilities.push(Ability::PowerCharge),
                                    Attr::CanHorde => horde = true,
                                    Attr::LegionaryName => legionary_name = true,
                                    Attr::InfiniteAmmo => infinite_ammo = true,
                                    Attr::NonScaling => non_scaling = true,
                                    _ => {}
                                }
                            }
                            if cant_hide {
                                abilities.push(Ability::CantHide)
                            }
                            if frighten_foot && frighten_mounted {
                                abilities.push(Ability::FrightenAll)
                            } else if frighten_foot {
                                abilities.push(Ability::FrightenFoot)
                            } else if frighten_mounted {
                                abilities.push(Ability::FrightenMounted)
                            }
                            silphium::model::Unit {
                                id: u.id.clone().into(),
                                key: u.key.clone().into(),
                                name: text.get(&u.key).cloned().unwrap_or(u.key.clone()).into(),
                                image: format!(
                                    "data/ui/units/{}/#{}.tga",
                                    f.id.to_lowercase(),
                                    u.key.to_lowercase()
                                )
                                .into(),
                                soldiers: u.stats.soldiers,
                                officers: u.stats.officers,
                                has_mount: u
                                    .stats
                                    .mount
                                    .as_ref()
                                    .map(|m| !m.contains("horse"))
                                    .unwrap_or(false),
                                formations: u.stats.formations.clone().into(),
                                hp: if u.stats.hp < 0 { 0 } else { u.stats.hp as u32 },
                                hp_mount: if u.stats.hp_mount < 0 {
                                    0
                                } else {
                                    u.stats.hp_mount as u32
                                },
                                primary_weapon: build_weapon(&u.stats.primary_weapon),
                                secondary_weapon: build_weapon(&u.stats.secondary_weapon),
                                defense: u.stats.defense,
                                defense_mount: u.stats.defense_mount,
                                heat: u.stats.heat,
                                ground_bonus: u.stats.ground_bonus,
                                morale: u.stats.morale,
                                discipline: u.stats.discipline,
                                turns: u.stats.turns,
                                cost: u.stats.cost,
                                upkeep: u.stats.upkeep,
                                eras: vec![].into(), // TODO

                                stamina,
                                inexhaustible,
                                infinite_ammo,
                                scaling: !non_scaling,
                                horde,
                                general,
                                mercenary,
                                legionary_name,

                                abilities: abilities.into(),
                            }
                        })
                        .filter(|u| !u.mercenary)
                        .collect(),
                },
            )
        })
        .collect();

    factions
}

fn build_weapon(weapon: &export_descr_unit::Weapon) -> Option<silphium::model::Weapon> {
    if weapon.weapon_type == "no" {
        return None;
    }

    let mut class = if weapon.missile != "no" {
        WeaponType::Missile
    } else {
        WeaponType::Melee
    };
    let mut armor_piercing = false;
    let mut body_piercing = false;
    let mut pre_charge = false;
    let mut launching = false;
    let mut area = false;
    let mut fire = false;
    let mut spear_bonus = 0;
    for attr in weapon.attributes.iter() {
        match attr {
            export_descr_unit::WeaponAttr::ArmorPiercing => armor_piercing = true,
            export_descr_unit::WeaponAttr::BodyPiercing => body_piercing = true,
            export_descr_unit::WeaponAttr::Spear
            | export_descr_unit::WeaponAttr::LongPike
            | export_descr_unit::WeaponAttr::ShortPike
            | export_descr_unit::WeaponAttr::LightSpear => class = WeaponType::Spear,
            export_descr_unit::WeaponAttr::Precharge => pre_charge = true,
            export_descr_unit::WeaponAttr::Thrown => class = WeaponType::Thrown,
            export_descr_unit::WeaponAttr::Launching => launching = true,
            export_descr_unit::WeaponAttr::Area => area = true,
            export_descr_unit::WeaponAttr::SpearBonus(n) => spear_bonus = *n,
            export_descr_unit::WeaponAttr::Fire => fire = true,
            _ => {}
        }
    }

    Some(silphium::model::Weapon {
        class,
        factor: weapon.factor,
        is_missile: weapon.missile != "no",
        charge: weapon.charge,
        range: weapon.range,
        ammo: weapon.ammo,
        lethality: weapon.lethality,
        armor_piercing,
        body_piercing,
        pre_charge,
        launching,
        area,
        fire,
        spear_bonus,
    })
}

fn is_general(unit: &export_descr_unit::Unit) -> bool {
    unit.stats.attributes.contains(&Attr::GeneralUnit)
}

fn require_ownership(unit: &export_descr_unit::Unit) -> Requires {
    Requires::Factions(unit.ownership.clone())
}

fn build_requires(
    raw_buildings: Vec<Building>,
    unit_map: &IndexMap<String, export_descr_unit::Unit>,
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
        Requires::Alias(id) => do_evaluate(
            aliases.get(id).expect(&format!("invalid alias: {id}")),
            aliases,
            eval,
        ),
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

async fn parse_text(path: PathBuf, mode: ParserMode) -> Result<HashMap<String, String>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf16le_lossy(&buf).replace(BOM, "");
    text::parse(data, mode)
}

async fn parse_descr_mercenaries(path: PathBuf, mode: ParserMode) -> Result<Vec<Pool>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_mercenaries::parse(data, mode)
}

async fn parse_descr_regions(path: PathBuf, mode: ParserMode) -> Result<Vec<Region>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_regions::parse(data, mode)
}

async fn parse_descr_sm_factions(
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<descr_sm_factions::Faction>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_sm_factions::parse(data, mode)
}

async fn parse_export_descr_unit(
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<export_descr_unit::Unit>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    export_descr_unit::parse(data, mode)
}

async fn parse_export_descr_buildings(
    path: PathBuf,
    mode: ParserMode,
) -> Result<(HashMap<String, Requires>, Vec<Building>)> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    export_descr_buildings::parse(data, mode)
}

const BOM: &str = "\u{feff}";
