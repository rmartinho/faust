use std::collections::{HashMap, HashSet};

use implicit_clone::unsync::{IArray, IString};
use indexmap::IndexMap;
use silphium::model::{self, Ability, MountType, UnitClass, WeaponType};

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

pub fn build_model(
    cfg: &Config,
    raw_units: Vec<export_descr_unit::Unit>,
    mut raw_factions: Vec<descr_sm_factions::Faction>,
    raw_regions: Vec<Region>,
    raw_pools: Vec<Pool>,
    raw_buildings: Vec<Building>,
    aliases: HashMap<String, Requires>,
    text: HashMap<String, String>,
    strat: HashMap<String, usize>,
    mounts: HashMap<String, Mount>,
    models: HashMap<String, Model>,
) -> (
    IndexMap<IString, model::Faction>,
    IndexMap<IString, model::Region>,
    IArray<model::Pool>,
) {
    let unit_map: IndexMap<_, _> = raw_units.into_iter().map(|u| (u.id.clone(), u)).collect();
    let requires = build_requires(&raw_buildings, &unit_map);
    let tech_levels = build_tech_levels(&raw_buildings);

    let regions = raw_regions
        .into_iter()
        .map(|r| {
            (
                r.id.clone().into(),
                model::Region {
                    id: r.id.into(),
                    legion: r.legion.map(Into::into),
                    color: r.color,
                    hidden_resources: r.hidden_resources.into_iter().map(Into::into).collect(),
                },
            )
        })
        .collect();

    let pools = raw_pools
        .into_iter()
        .map(|p| model::Pool {
            id: p.id.into(),
            regions: p.regions.into_iter().map(Into::into).collect(),
            units: p
                .units
                .into_iter()
                .map(|e| model::PoolEntry {
                    unit: e.id.into(),
                    exp: e.exp,
                    cost: e.cost,
                    replenish: e.replenish.into(),
                    max: e.max,
                    initial: e.initial,
                    restrict: e.restrict.into_iter().map(Into::into).collect(),
                })
                .collect(),
            map: "".into(),
        })
        .collect();

    raw_factions
        .extract_if(.., |f| !strat.contains_key(&f.id))
        .count();
    raw_factions.sort_by_key(|f| strat[&f.id]);
    let factions = raw_factions
        .into_iter()
        .filter(|f| !cfg.manifest.exclude.contains(&f.id))
        .map(|f| {
            let mut is_horde = false;
            let roster: IArray<_> = unit_map
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
                            Attr::CanRunAmok => abilities.push(Ability::CanRunAmok),
                            Attr::GeneralUnit => general = true,
                            Attr::CantabrianCircle => abilities.push(Ability::CantabrianCircle),
                            Attr::Command => abilities.push(Ability::Command),
                            Attr::Druid | Attr::ScreechingWomen => abilities.push(Ability::Chant),
                            Attr::MercenaryUnit => mercenary = true,
                            Attr::Hardy => stamina += 2,
                            Attr::VeryHardy => stamina += 4,
                            Attr::ExtremelyHardy => stamina += 8,
                            Attr::Inexhaustible => inexhaustible = true,
                            Attr::Warcry => abilities.push(Ability::Warcry),
                            Attr::PowerCharge => abilities.push(Ability::PowerCharge),
                            Attr::CanHorde => {
                                is_horde = true;
                                horde = true;
                            }
                            Attr::LegionaryName => legionary_name = true,
                            Attr::InfiniteAmmo => infinite_ammo = true,
                            Attr::NonScaling => non_scaling = true,
                            Attr::FreeUpkeep => is_militia = true,
                            Attr::Unique => is_unique = true,
                            Attr::Knight => abilities.push(Ability::Knight),
                            Attr::Gunpowder => {} // TODO?
                            Attr::FormedCharge => abilities.push(Ability::FormedCharge),
                            Attr::Stakes => abilities.push(Ability::Stakes),

                            _ => {}
                        }
                    }
                    let class = if is_general(u) {
                        UnitClass::General
                    } else if is_elephant(u, &mounts) {
                        UnitClass::Animal
                    } else if u.category.contains("cavalry") {
                        UnitClass::Cavalry
                    } else if u.category.contains("handler") {
                        UnitClass::Animal
                    } else if u.category.contains("siege") {
                        UnitClass::Artillery
                    } else if u.category.contains("ship") {
                        UnitClass::Ship
                    } else if u.class.contains("missile") {
                        UnitClass::Missile
                    } else if u.class.contains("spearmen") || has_spears(u) {
                        UnitClass::Spear
                    } else {
                        UnitClass::Sword
                    };
                    if cant_hide {
                        abilities.push(Ability::CantHide)
                    } else if hide_anywhere {
                        abilities.push(Ability::HideAnywhere)
                    } else if hide_forest {
                        abilities.push(Ability::HideImprovedForest)
                    } else if hide_grass {
                        abilities.push(Ability::HideLongGrass)
                    }
                    if frighten_foot && frighten_mounted {
                        abilities.push(Ability::FrightenAll)
                    } else if frighten_foot {
                        abilities.push(Ability::FrightenFoot)
                    } else if frighten_mounted {
                        abilities.push(Ability::FrightenMounted)
                    }
                    let move_speed = get_move_speed(cfg, u, &mounts, &models);
                    if class == UnitClass::Ship {
                        abilities.clear();
                    }
                    model::Unit {
                        id: u.id.clone().into(),
                        key: u.key.clone().into(),
                        name: text
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
                                f.id.to_lowercase(),
                                u.key.to_lowercase()
                            )
                        } else {
                            format!(
                                "data/ui/units/{}/#{}.tga",
                                f.id.to_lowercase(),
                                u.key.to_lowercase()
                            )
                        }
                        .into(),
                        soldiers: u.stats.soldiers,
                        officers: u.stats.officers,
                        mount: mount_type(u, &mounts),
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
                                        requires.get(&u.id).unwrap_or(&Requires::False),
                                        &aliases,
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

                        move_speed,

                        abilities: abilities.into(),
                        tech_level: tech_levels.get(&u.id).copied().unwrap_or(99),
                    }
                })
                .filter(|u| !u.mercenary)
                .collect();
            (
                f.id.clone().into(),
                model::Faction {
                    id: f.id.clone().into(),
                    name: text
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
                    eras: {
                        let redundant_eras = roster.iter().fold(
                            HashSet::from_iter(cfg.manifest.eras.keys().cloned()),
                            |s: HashSet<IString>, u| &s & &u.eras.iter().collect(),
                        );
                        let unique_eras = &HashSet::from_iter(cfg.manifest.eras.keys().cloned())
                            - &redundant_eras;
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
        })
        .collect();

    (factions, regions, pools)
}

fn get_move_skeleton<'a>(
    unit: &export_descr_unit::Unit,
    mounts: &HashMap<String, Mount>,
    models: &'a HashMap<String, Model>,
) -> Option<&'a str> {
    if is_ship(unit) {
        return None;
    }
    let model = get_move_model(unit, mounts);
    model
        .and_then(|m| models.get(m))
        .map(|m| m.skeleton.as_str())
}

fn get_move_model<'a>(
    unit: &'a export_descr_unit::Unit,
    mounts: &'a HashMap<String, Mount>,
) -> Option<&'a str> {
    if let Some(mount) = get_mount(unit, mounts) {
        let mount = if is_chariot(unit, mounts) {
            &mounts[mount.horse.as_ref().unwrap()]
        } else {
            mount
        };
        mount.model.as_ref().map(String::as_str)
    } else {
        Some(&unit.stats.soldier_model)
    }
}

fn get_move_speed(
    cfg: &Config,
    unit: &export_descr_unit::Unit,
    mounts: &HashMap<String, Mount>,
    models: &HashMap<String, Model>,
) -> Option<u32> {
    get_move_skeleton(unit, mounts, models).and_then(|s| {
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
        WeaponType::Melee
    } else if weapon.tech_type.contains("gunpowder") {
        WeaponType::Gunpowder
    } else {
        WeaponType::Missile
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
    mounts: &'a HashMap<String, Mount>,
) -> Option<&'a Mount> {
    unit.stats
        .mount
        .as_ref()
        .and_then(|mount| mounts.get(mount))
}

fn is_chariot(unit: &export_descr_unit::Unit, mounts: &HashMap<String, Mount>) -> bool {
    get_mount(unit, mounts).is_some_and(|mount| mount.class == MountClass::Chariot)
}

fn can_horde(unit: &export_descr_unit::Unit) -> bool {
    unit.stats.attributes.contains(&Attr::CanHorde)
}

fn is_elephant(unit: &export_descr_unit::Unit, mounts: &HashMap<String, Mount>) -> bool {
    get_mount(unit, mounts).is_some_and(|mount| mount.class == MountClass::Elephant)
}

fn mount_type(unit: &export_descr_unit::Unit, mounts: &HashMap<String, Mount>) -> MountType {
    get_mount(unit, mounts).map_or(MountType::Foot, |mount| {
        if mount.class == MountClass::Horse {
            MountType::Horse
        } else {
            MountType::Other
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
    aliases: &HashMap<String, Requires>,
) -> bool {
    evaluate(req, aliases, &Evaluator::region(region))
}
