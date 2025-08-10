use std::collections::{BTreeSet, HashMap, HashSet};

use implicit_clone::unsync::{IArray, IString};
use indexmap::IndexMap;
use silphium::model;

use crate::{
    args::Config,
    parse::{
        descr_mercenaries::Pool,
        descr_model_battle::Model,
        descr_mount::{Mount, MountClass},
        descr_regions::Region,
        descr_sm_factions,
        eval::{Evaluator, evaluate},
        export_descr_buildings::{Building, Requires},
        export_descr_unit::{self, Attr, WeaponAttr},
    },
};

pub struct RawModel {
    pub units: Vec<export_descr_unit::Unit>,
    pub factions: Vec<descr_sm_factions::Faction>,
    pub regions: Vec<Region>,
    pub pools: Vec<Pool>,
    pub buildings: Vec<Building>,
    pub require_aliases: HashMap<String, Requires>,
    pub text: HashMap<String, String>,
    pub strat: HashMap<String, usize>,
    pub mounts: HashMap<String, Mount>,
    pub models: HashMap<String, Model>,
}

struct IntermediateModel {
    unit_map: IndexMap<String, export_descr_unit::Unit>,
    factions: Vec<descr_sm_factions::Faction>,
    regions: Vec<Region>,
    pools: Vec<Pool>,
    buildings: Vec<Building>,
    require_aliases: HashMap<String, Requires>,
    text: HashMap<String, String>,
    strat: HashMap<String, usize>,
    mounts: HashMap<String, Mount>,
    models: HashMap<String, Model>,
    requires: HashMap<String, Requires>,
    tech_levels: HashMap<String, u32>,
}

pub fn build_model(
    cfg: &Config,
    raw: RawModel,
) -> (
    IndexMap<IString, model::Faction>,
    IndexMap<IString, model::Region>,
    IArray<model::Pool>,
    IArray<model::Aor>,
) {
    let unit_map: IndexMap<_, _> = raw.units.into_iter().map(|u| (u.id.clone(), u)).collect();
    let requires = build_requires(&raw.buildings, &unit_map);
    let tech_levels = build_tech_levels(&raw.buildings);
    let mut raw = IntermediateModel {
        unit_map,
        factions: raw.factions,
        regions: raw.regions,
        pools: raw.pools,
        buildings: raw.buildings,
        require_aliases: raw.require_aliases,
        text: raw.text,
        strat: raw.strat,
        mounts: raw.mounts,
        models: raw.models,
        requires,
        tech_levels,
    };

    let regions = raw
        .regions
        .iter()
        .map(|r| {
            (
                r.id.clone().into(),
                model::Region {
                    id: r.id.clone().into(),
                    legion: r.legion.as_ref().map(|s| s.clone().into()),
                    color: r.color,
                    hidden_resources: r
                        .hidden_resources
                        .iter()
                        .map(|s| s.clone().into())
                        .collect(),
                },
            )
        })
        .collect();
    let pools = raw
        .pools
        .iter()
        .enumerate()
        .map(|(i, p)| build_pool(p, i, cfg, &raw))
        .collect();

    let aors = calculate_aors(cfg, &raw);

    raw.factions
        .extract_if(.., |f| !raw.strat.contains_key(&f.id))
        .count();
    raw.factions.sort_by_key(|f| raw.strat[&f.id]);
    let factions = raw
        .factions
        .iter()
        .filter(|f| !cfg.manifest.exclude.contains(&f.id))
        .map(|f| build_faction(f, cfg, &raw, aors.as_slice()))
        .collect();

    (factions, regions, pools, aors)
}

fn build_faction(
    f: &descr_sm_factions::Faction,
    cfg: &Config,
    raw: &IntermediateModel,
    aors: &[model::Aor],
) -> (IString, model::Faction) {
    let mut is_horde = false;
    let mut has_aors = false;
    let roster: IArray<_> = raw
        .unit_map
        .values()
        .filter(|u| {
            available_to_faction(
                raw.requires.get(&u.id).unwrap_or(&Requires::False),
                &f,
                &raw.require_aliases,
            )
        })
        .map(|u| {
            let u = build_unit(u, cfg, &f.id, raw, aors);
            if u.horde {
                is_horde = true;
            }
            if u.is_regional {
                has_aors = true;
            }
            u
        })
        .filter(|u| !u.mercenary)
        .collect();
    (
        f.id.clone().into(),
        model::Faction {
            id: f.id.clone().into(),
            name: raw
                .text
                .get(&f.name.to_lowercase())
                .cloned()
                .unwrap_or(f.name.clone())
                .trim()
                .to_string()
                .into(),
            image: f
                .logo
                .to_str()
                .expect("invalid file name")
                .to_lowercase()
                .into(),
            alias: cfg
                .manifest
                .aliases
                .iter()
                .find(|(_, v)| *v == &f.id)
                .map(|(k, _)| k.clone()),
            is_horde,
            has_aors,
            eras: {
                let redundant_eras = roster.iter().fold(
                    HashSet::from_iter(cfg.manifest.eras.keys().cloned()),
                    |s: HashSet<IString>, u| &s & &u.eras.iter().collect(),
                );
                let unique_eras =
                    &HashSet::from_iter(cfg.manifest.eras.keys().cloned()) - &redundant_eras;
                cfg.manifest
                    .eras
                    .iter()
                    .map(|(id, _)| id)
                    .filter(|id| unique_eras.contains(id.as_ref()))
                    .cloned()
                    .collect()
            },
            roster,
        },
    )
}

fn build_unit(
    u: &export_descr_unit::Unit,
    cfg: &Config,
    f_id: &str,
    raw: &IntermediateModel,
    aors: &[model::Aor],
) -> model::Unit {
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
    let mut is_militia = false;
    let mut is_unique = false;
    let mut hide_forest = false;
    let mut hide_grass = false;
    let mut hide_anywhere = false;
    for attr in u.stats.attributes.iter() {
        match attr {
            Attr::HideForest => cant_hide = false,
            Attr::HideImprovedForest => {
                cant_hide = false;
                hide_forest = true;
            }
            Attr::HideLongGrass => {
                cant_hide = false;
                hide_grass = true;
            }
            Attr::HideAnywhere => {
                cant_hide = false;
                hide_anywhere = true;
            }
            Attr::FrightenFoot => frighten_foot = true,
            Attr::FrightenMounted => frighten_mounted = true,
            Attr::CanRunAmok => abilities.push(model::Ability::CanRunAmok),
            Attr::GeneralUnit => general = true,
            Attr::CantabrianCircle => abilities.push(model::Ability::CantabrianCircle),
            Attr::Command => abilities.push(model::Ability::Command),
            Attr::Druid | Attr::ScreechingWomen => abilities.push(model::Ability::Chant),
            Attr::MercenaryUnit => mercenary = true,
            Attr::Hardy => stamina += 2,
            Attr::VeryHardy => stamina += 4,
            Attr::ExtremelyHardy => stamina += 8,
            Attr::Inexhaustible => inexhaustible = true,
            Attr::Warcry => abilities.push(model::Ability::Warcry),
            Attr::PowerCharge => abilities.push(model::Ability::PowerCharge),
            Attr::CanHorde => horde = true,
            Attr::LegionaryName => legionary_name = true,
            Attr::InfiniteAmmo => infinite_ammo = true,
            Attr::NonScaling => non_scaling = true,
            Attr::FreeUpkeep => is_militia = true,
            Attr::Unique => is_unique = true,
            Attr::Knight => abilities.push(model::Ability::Knight),
            Attr::Gunpowder => {} // TODO?
            Attr::FormedCharge => abilities.push(model::Ability::FormedCharge),
            Attr::Stakes => abilities.push(model::Ability::Stakes),

            _ => {}
        }
    }
    let class = if is_general(u) {
        model::UnitClass::General
    } else if is_elephant(u, cfg, raw) {
        model::UnitClass::Animal
    } else if u.category.contains("cavalry") {
        model::UnitClass::Cavalry
    } else if u.category.contains("handler") {
        model::UnitClass::Animal
    } else if u.category.contains("siege") {
        model::UnitClass::Artillery
    } else if u.category.contains("ship") {
        model::UnitClass::Ship
    } else if u.class.contains("missile") {
        model::UnitClass::Missile
    } else if u.class.contains("spearmen") || has_spears(u) {
        model::UnitClass::Spear
    } else {
        model::UnitClass::Sword
    };
    if cant_hide {
        abilities.push(model::Ability::CantHide)
    } else if hide_anywhere {
        abilities.push(model::Ability::HideAnywhere)
    } else if hide_forest {
        abilities.push(model::Ability::HideImprovedForest)
    } else if hide_grass {
        abilities.push(model::Ability::HideLongGrass)
    }
    if frighten_foot && frighten_mounted {
        abilities.push(model::Ability::FrightenAll)
    } else if frighten_foot {
        abilities.push(model::Ability::FrightenFoot)
    } else if frighten_mounted {
        abilities.push(model::Ability::FrightenMounted)
    }
    let move_speed = get_move_speed(u, cfg, raw);
    if class == model::UnitClass::Ship {
        abilities.clear();
    }

    let id = u.id.clone().into();
    let is_regional = aors
        .iter()
        .any(|aor| aor.faction == f_id && aor.units.contains(&id));

    model::Unit {
        id,
        key: u.key.clone().into(),
        name: raw
            .text
            .get(&u.key.to_lowercase())
            .cloned()
            .unwrap_or(u.key.clone())
            .trim()
            .to_string()
            .into(),
        class,
        image: if cfg.manifest.unit_info_images {
            format!(
                "data/ui/unit_info/{}/{}_info.tga",
                if f_id == "mercs" {
                    "merc".into()
                } else {
                    f_id.to_lowercase()
                },
                u.key.to_lowercase()
            )
        } else {
            format!(
                "data/ui/units/{}/#{}.tga",
                f_id.to_lowercase(),
                u.key.to_lowercase()
            )
        }
        .into(),
        soldiers: u.stats.soldiers,
        officers: u.stats.officers,
        mount: mount_type(u, cfg, raw),
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
        eras: {
            cfg.manifest
                .eras
                .iter()
                .filter_map(|(id, e)| {
                    if evaluate(
                        raw.requires.get(&u.id).unwrap_or(&Requires::False),
                        &raw.require_aliases,
                        &e.evaluator,
                    ) {
                        Some(id.clone())
                    } else {
                        None
                    }
                })
                .collect()
        },

        stamina,
        inexhaustible,
        infinite_ammo,
        scaling: !non_scaling,
        horde,
        general,
        mercenary,
        legionary_name,
        is_militia,
        is_unique,

        is_regional,

        move_speed,

        abilities: abilities.into(),
        tech_level: raw.tech_levels.get(&u.id).copied().unwrap_or(99),
    }
}

fn build_pool(p: &Pool, index: usize, cfg: &Config, raw: &IntermediateModel) -> model::Pool {
    model::Pool {
        id: p.id.clone().into(),
        name: cfg
            .manifest
            .pools
            .get(index)
            .cloned()
            .unwrap_or_else(|| p.id.clone().into()),
        regions: p.regions.iter().map(|s| s.clone().into()).collect(),
        units: p
            .units
            .iter()
            .map(|e| {
                let u = &raw.unit_map[&e.id];
                let mut unit = build_unit(u, cfg, "mercs", raw, &[]);
                unit.cost = e.cost;
                model::PoolEntry {
                    unit,
                    exp: e.exp,
                    replenish: e.replenish.into(),
                    max: e.max,
                    initial: e.initial,
                    restrict: e.restrict.iter().map(|s| s.clone().into()).collect(),
                }
            })
            .collect(),
        map: format!("pool-{}", index + 1).into(),
    }
}

fn get_move_skeleton<'a>(
    unit: &export_descr_unit::Unit,
    cfg: &Config,
    raw: &'a IntermediateModel,
) -> Option<&'a str> {
    if is_ship(unit) {
        return None;
    }
    let model = get_move_model(unit, cfg, raw);
    model
        .and_then(|m| raw.models.get(m))
        .map(|m| m.skeleton.as_str())
}

fn get_move_model<'a>(
    unit: &'a export_descr_unit::Unit,
    cfg: &Config,
    raw: &'a IntermediateModel,
) -> Option<&'a str> {
    if let Some(mount) = get_mount(unit, cfg, raw) {
        let mount = if is_chariot(unit, cfg, raw) {
            &raw.mounts[mount.horse.as_ref().unwrap()]
        } else {
            mount
        };
        mount.model.as_ref().map(String::as_str)
    } else {
        Some(&unit.stats.soldier_model)
    }
}

fn get_move_speed(
    unit: &export_descr_unit::Unit,
    cfg: &Config,
    raw: &IntermediateModel,
) -> Option<u32> {
    get_move_skeleton(unit, cfg, raw).and_then(|s| {
        cfg.manifest.speeds.get(s).copied().or_else(|| {
            SKELETON_SPEED.iter().find_map(|&(sk, sp)| {
                (sk == s).then_some((sp as f64 * unit.stats.speed_mod).round() as u32)
            })
        })
    })
}

const SKELETON_SPEED: &[(&str, u32)] = &[
    ("fs_slow_spearman", 26),
    ("fs_spearman", 30),
    ("fs_semi_fast_spearman", 32),
    ("fs_dagger", 30),
    ("fs_semi_fast_dagger", 32),
    ("fs_slow_swordsman", 26),
    ("fs_swordsman", 30),
    ("fs_semi_fast_swordsman", 32),
    ("fs_archer", 30),
    ("fs_semi_fast_archer", 32),
    ("fs_javelinman", 30),
    ("fs_semi_fast_javelinman", 32),
    ("fs_2handed", 30),
    ("fs_2handed_berserker", 32),
    ("fs_slinger_new", 35),
    ("fs_standard_bearer", 30),
    ("fs_indian_elephant", 39),
    ("fs_african_elephant", 39),
    ("fs_forest_elephant", 39),
    ("fs_indian_giant_elephant", 39),
    ("fs_camel", 40),
    ("fs_cataphract_horse", 41),
    ("fs_medium_horse", 50),
    ("fs_horse", 54),
    ("fs_fast_horse", 62),
];

fn build_weapon(weapon: &export_descr_unit::Weapon) -> Option<model::Weapon> {
    if weapon.weapon_type == "no" {
        return None;
    }

    let mut class = if weapon.missile == "no" {
        model::WeaponType::Melee
    } else if weapon.tech_type.contains("gunpowder") {
        model::WeaponType::Gunpowder
    } else {
        model::WeaponType::Missile
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
            | export_descr_unit::WeaponAttr::LightSpear => class = model::WeaponType::Spear,
            export_descr_unit::WeaponAttr::Precharge => pre_charge = true,
            export_descr_unit::WeaponAttr::Thrown => class = model::WeaponType::Thrown,
            export_descr_unit::WeaponAttr::Launching => launching = true,
            export_descr_unit::WeaponAttr::Area => area = true,
            export_descr_unit::WeaponAttr::SpearBonus(n) => spear_bonus = *n,
            export_descr_unit::WeaponAttr::Fire => fire = true,
            _ => {}
        }
    }

    Some(model::Weapon {
        class,
        factor: weapon.factor,
        is_missile: weapon.missile != "no",
        charge: weapon.charge,
        range: weapon.range,
        ammo: weapon.ammo,
        lethality: if weapon.lethality > 1.0 {
            1.0
        } else {
            weapon.lethality
        },
        armor_piercing,
        body_piercing,
        pre_charge,
        launching,
        area,
        fire,
        spear_bonus,
    })
}

fn is_ship(unit: &export_descr_unit::Unit) -> bool {
    unit.category.contains("ship")
}

fn is_general(unit: &export_descr_unit::Unit) -> bool {
    unit.stats.attributes.contains(&Attr::GeneralUnit)
}

fn has_mount(unit: &export_descr_unit::Unit) -> bool {
    unit.stats.mount.is_some()
}

fn get_mount<'a>(
    unit: &export_descr_unit::Unit,
    _cfg: &Config,
    raw: &'a IntermediateModel,
) -> Option<&'a Mount> {
    unit.stats
        .mount
        .as_ref()
        .and_then(|mount| raw.mounts.get(mount))
}

fn is_chariot(unit: &export_descr_unit::Unit, cfg: &Config, raw: &IntermediateModel) -> bool {
    get_mount(unit, cfg, raw).is_some_and(|mount| mount.class == MountClass::Chariot)
}

fn can_horde(unit: &export_descr_unit::Unit) -> bool {
    unit.stats.attributes.contains(&Attr::CanHorde)
}

fn is_elephant(unit: &export_descr_unit::Unit, cfg: &Config, raw: &IntermediateModel) -> bool {
    get_mount(unit, cfg, raw).is_some_and(|mount| mount.class == MountClass::Elephant)
}

fn mount_type(
    unit: &export_descr_unit::Unit,
    cfg: &Config,
    raw: &IntermediateModel,
) -> model::MountType {
    get_mount(unit, cfg, raw).map_or(model::MountType::Foot, |mount| {
        if mount.class == MountClass::Horse {
            model::MountType::Horse
        } else if mount.class == MountClass::Camel {
            model::MountType::Camel
        } else if mount.class == MountClass::Elephant {
            model::MountType::Elephant
        } else if mount.class == MountClass::Chariot {
            model::MountType::Chariot
        } else {
            model::MountType::Other
        }
    })
}

fn has_spears(unit: &export_descr_unit::Unit) -> bool {
    unit.stats
        .primary_weapon
        .attributes
        .iter()
        .find(|a| {
            matches!(
                a,
                WeaponAttr::LightSpear
                    | WeaponAttr::Spear
                    | WeaponAttr::ShortPike
                    | WeaponAttr::LongPike
                    | WeaponAttr::SpearBonus(_)
            )
        })
        .is_some()
}

fn general_upgrade_event(unit: &export_descr_unit::Unit) -> Option<String> {
    unit.stats
        .attributes
        .iter()
        .find_map(|attr| match attr {
            Attr::GeneralUnitUpgrade(s) => Some(s),
            _ => None,
        })
        .cloned()
}

fn require_ownership(unit: &export_descr_unit::Unit) -> Requires {
    Requires::Factions(unit.ownership.clone())
}

fn require_no_events<'a>(events: impl Iterator<Item = &'a String>) -> Requires {
    Requires::And(
        events
            .map(|e| Requires::Not(Box::new(Requires::MajorEvent(e.clone()))))
            .collect(),
    )
}

fn tech_level(s: impl AsRef<str>) -> u32 {
    match s.as_ref() {
        "village" => 0,
        "town" => 1,
        "large_town" => 2,
        "city" => 3,
        "large_city" => 4,
        "huge_city" => 5,
        _ => 99,
    }
}

fn build_tech_levels(raw_buildings: &Vec<Building>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    raw_buildings
        .into_iter()
        .flat_map(|b| {
            b.caps
                .iter()
                .map(move |r| (r.unit.clone(), tech_level(&b.min)))
        })
        .for_each(|(u, level)| {
            map.entry(u)
                .and_modify(|old| {
                    if *old > level {
                        *old = level;
                    }
                })
                .or_insert(level);
        });
    map
}

fn build_requires(
    raw_buildings: &Vec<Building>,
    unit_map: &IndexMap<String, export_descr_unit::Unit>,
) -> HashMap<String, Requires> {
    let general_events: HashSet<_> = unit_map
        .values()
        .filter_map(|u| general_upgrade_event(u))
        .collect();
    let unupgraded_requires = require_no_events(general_events.iter());

    raw_buildings
        .into_iter()
        .flat_map(|b| {
            b.caps.iter().map(move |r| {
                let owners = require_ownership(&unit_map[&r.unit]);
                (
                    r.unit.clone(),
                    Requires::And(vec![r.req.clone(), b.req.clone(), owners]),
                )
            })
        })
        .chain(unit_map.values().filter(|&u| is_general(u)).map(|u| {
            (
                u.id.clone(),
                Requires::And(vec![
                    require_ownership(u),
                    general_upgrade_event(u)
                        .map(|e| Requires::MajorEvent(e.clone()))
                        .unwrap_or(unupgraded_requires.clone()),
                ]),
            )
        }))
        .chain(
            unit_map
                .values()
                .filter(|&u| can_horde(u))
                .map(|u| (u.id.clone(), require_ownership(u))),
        )
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
    faction: Option<&descr_sm_factions::Faction>,
    aliases: &HashMap<String, Requires>,
) -> bool {
    evaluate(req, aliases, &Evaluator::region(region, faction))
}

fn calculate_aors<'a>(cfg: &Config, raw: &'a IntermediateModel) -> IArray<model::Aor> {
    let mut unit_aors = HashMap::new();
    for region in raw.regions.iter() {
        for (unit, req) in raw.requires.iter() {
            if available_in_region(req, &region, None, &raw.require_aliases) {
                unit_aors
                    .entry(unit.clone())
                    .or_insert_with(BTreeSet::new)
                    .insert(region.id.as_str());
            }
        }
    }
    let all_regions: BTreeSet<_> = raw.regions.iter().map(|r| r.id.as_str()).collect();
    let aor_units: BTreeSet<_> = unit_aors
        .iter()
        .filter(|(_, set)| set.len() > 0 && set.len() < all_regions.len())
        .map(|(k, _)| k)
        .collect();
    let aors: BTreeSet<_> = unit_aors
        .values()
        .filter(|set| set.len() > 0 && set.len() < all_regions.len())
        .cloned()
        .collect();
    let aors: BTreeSet<Vec<IString>> = raw
        .regions
        .iter()
        .filter_map(|r| {
            aors.iter()
                .filter(|aor| aor.contains(r.id.as_str()))
                .cloned()
                .reduce(|l, r| &l & &r)
                .map(|set| set.into_iter().map(|s| s.to_string().into()).collect())
        })
        .collect();
    let region_map: &HashMap<_, _> = &raw.regions.iter().map(|r| (r.id.as_str(), r)).collect();
    aors.into_iter()
        .enumerate()
        .flat_map(|(i, regions)| {
            let regions: IArray<IString> = regions.into_iter().map(Into::into).collect();
            let aor_units = &aor_units;
            raw.factions.iter().filter_map(move |f| {
                let units: IArray<IString> = aor_units
                    .iter()
                    .filter_map(|u| {
                        let req = &raw.requires.get(u.as_str()).unwrap_or(&Requires::None);
                        regions
                            .iter()
                            .all(|r| {
                                available_in_region(
                                    req,
                                    region_map[r.as_str()],
                                    Some(f),
                                    &raw.require_aliases,
                                )
                            })
                            .then(|| (*u).clone().into())
                    })
                    .collect();

                (units.len() > 0).then(|| model::Aor {
                    name: cfg.manifest.aors.get(i).cloned().unwrap_or_default(),
                    map: format!("aor-{}", i + 1).into(),
                    faction: f.id.clone().into(),
                    units: units,
                    regions: regions.clone(),
                })
            })
        })
        .collect()
}
