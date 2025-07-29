use std::collections::{HashMap, HashSet};

use implicit_clone::unsync::{IArray, IString};
use indexmap::IndexMap;
use silphium::model::{Ability, UnitClass, WeaponType};

use crate::{
    args::Config,
    parse::{
        descr_mercenaries::Pool,
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
    _raw_regions: Vec<Region>,
    _raw_pools: Vec<Pool>,
    raw_buildings: Vec<Building>,
    aliases: HashMap<String, Requires>,
    text: HashMap<String, String>,
    strat: HashMap<String, usize>,
) -> IndexMap<IString, silphium::model::Faction> {
    let unit_map: IndexMap<_, _> = raw_units.into_iter().map(|u| (u.id.clone(), u)).collect();
    let requires = build_requires(&raw_buildings, &unit_map);
    let tech_levels = build_tech_levels(&raw_buildings);

    raw_factions
        .extract_if(.., |f| !strat.contains_key(&f.id))
        .count();
    raw_factions.sort_by_key(|f| strat[&f.id]);
    let factions = raw_factions
        .into_iter()
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
                        name: text
                            .get(&u.key.to_lowercase())
                            .cloned()
                            .unwrap_or(u.key.clone())
                            .trim()
                            .to_string()
                            .into(),
                        class: if is_general(u) {
                            UnitClass::General
                        } else if let Some(mount) = &u.stats.mount
                            && mount.contains("elephant")
                        {
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
                        } else if u.class.contains("spearmen") {
                            UnitClass::Spear
                        } else if has_spears(u) {
                            UnitClass::Spear
                        } else {
                            UnitClass::Sword
                        },
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

                        abilities: abilities.into(),
                        tech_level: tech_levels.get(&u.id).copied().unwrap_or(99),
                    }
                })
                .filter(|u| !u.mercenary)
                .collect();
            (
                f.id.clone().into(),
                silphium::model::Faction {
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
