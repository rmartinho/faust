use implicit_clone::{ImplicitClone, unsync::IArray};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    components::Text,
    model::{Ability, Defense, Discipline, Formation, Unit, UnitClass, Weapon, WeaponType},
};

#[derive(PartialEq, Clone, ImplicitClone, Default)]
pub struct UnitFilter {
    pub era: Option<AttrValue>,
    pub horde: Option<bool>,
}

impl UnitFilter {
    fn apply(&self, unit: &Unit) -> bool {
        (if let Some(ref era) = self.era {
            unit.eras.contains(era)
        } else {
            true
        }) && (if let Some(horde) = self.horde {
            unit.horde == horde
        } else {
            true
        })
    }
}

#[autoprops]
#[function_component(FactionRoster)]
pub fn faction_roster(roster: IArray<Unit>, filter: UnitFilter) -> Html {
    let mut roster: Vec<_> = roster.iter().filter(|unit| filter.apply(unit)).collect();
    roster.sort_by_key(|u| (u.tech_level, u.upkeep, u.cost));
    let roster: &IArray<_> = &roster.into();

    let groups = UnitClass::all()
        .into_iter()
        .map(|group| html! { <RosterGroup {roster} {group} /> });

    html! {
      <div class="roster">
        {for groups}
      </div>
    }
}

#[autoprops]
#[function_component(RosterGroup)]
pub fn roster_group(roster: IArray<Unit>, group: UnitClass) -> Html {
    let cards: Vec<_> = roster
        .iter()
        .filter(|u| u.class == group)
        .map(|unit| {
            html! {
              <UnitCard {unit}/>
            }
        })
        .collect();

    let icon = format!("/icons/class/{group}.svg");
    let title = match group {
        UnitClass::Sword => "Blade infantry",
        UnitClass::Spear => "Spear infantry",
        UnitClass::Missile => "Missile infantry",
        UnitClass::Cavalry => "Cavalry",
        UnitClass::General => "General bodyguards",
        UnitClass::Animal => "Animals",
        UnitClass::Artillery => "Artillery",
        UnitClass::Ship => "Navy",
    };

    html! {
      <>
        if cards.len() > 0 {
          <div class="roster-group">
            <div class="legend">
              <img class="group" src={icon} {title} />
            </div>
            <div class="unit-cards">
              {for cards}
            </div>
          </div>
        }
      </>
    }
}

#[autoprops]
#[function_component(UnitCard)]
pub fn unit_card(unit: Unit) -> Html {
    use std::fmt::Write as _;

    let mut soldiers_title = format!("{} soldiers", unit.soldiers);
    if unit.officers > 0 {
        let _ = write!(
            soldiers_title,
            "\n{} {}",
            unit.officers,
            if unit.officers > 1 {
                "officers"
            } else {
                "officer"
            }
        );
    }

    let formations = unit.formations.iter().map(|f| {
        let title = match &f {
            Formation::Square => "Square formation",
            Formation::Horde => "Wedge formation",
            Formation::Phalanx => "Phalanx",
            Formation::Testudo => "Testudo",
            Formation::Wedge => "Horde",
            Formation::Schiltrom => "Schiltrom",
            Formation::ShieldWall => "Shield wall",
        };
        html! {
          <img src={format!("/icons/formation/{f}.svg")} {title} />
        }
    });

    let discipline_tooltip = match unit.discipline {
        Discipline::Low => "Low discipline",
        Discipline::Normal => "Normal discipline",
        Discipline::Disciplined => "Disciplined",
        Discipline::Impetuous => "May charge without orders",
        Discipline::Berserker => "Berserker",
    };

    let abilities = unit.abilities.iter().map(|ab| {
        let title = match ab {
            Ability::CantHide => "Cannot hide",
            Ability::HideImprovedForest => "Can hide well in forests",
            Ability::HideLongGrass => "Can hide in long grass",
            Ability::HideAnywhere => "Can hide anywhere",
            Ability::FrightenFoot => "Frightens nearby infantry",
            Ability::FrightenMounted => "Frightens nearby cavalry",
            Ability::FrightenAll => "Frightens nearby units",
            Ability::CanRunAmok => "Can run amok",
            Ability::CantabrianCircle => "Can form Cantabrian circle",
            Ability::Command => "Inspires nearby units",
            Ability::Warcry => "Can perform warcry to increase attack",
            Ability::PowerCharge => "Powerful charge",
            Ability::Chant => "Can chant to affect morale",
            Ability::FormedCharge => "Can do formed charge",
            Ability::Stakes => "Can lay defensive stakes",
        };
        html! {
          <img class="ability" src={format!("/icons/ability/{}.svg", ab)} {title} />
        }
    });

    html! {
      <div class="unit-card">
        <div class="name"><Text text={&unit.name} /></div>
        <div class="frame">
          <img class="image" title={&unit.name} src={&unit.image} />
          <div class="size-row">
            <img class="icon" src="/icons/stat/soldiers.svg" title="Soldiers" />
            <div class="size" title={soldiers_title}>
              <span class="soldiers">{ unit.soldiers }</span>
              if unit.officers > 0 {
                <span class="officers">{ unit.officers }</span>
              }
              <div class="formations">
                {for formations}
              </div>
            </div>
          </div>
          <div class="cost-row">
            <img class="icon" src="/icons/stat/cost.svg" title="Recruitment cost" />
            <div class="cost" title={format!("Cost: {}", unit.cost)}>
              <span>{ unit.cost }</span>
              if unit.turns > 1 {
                <div class="turns" title={format!("{} turns", unit.turns)}>
                  <img class="attribute" src="/icons/attribute/turns.svg" />
                  <span>{ unit.turns }</span>
                </div>
              }
            </div>
          </div>
          <div class="upkeep-row">
            <img class="icon" src="/icons/stat/upkeep.svg" title="Upkeep cost" />
            <div class="upkeep" title={format!("Upkeep: {}", unit.upkeep)}>
              <span>{ unit.upkeep }</span>
            </div>
          </div>
          <div class="mental-row">
            <img class="icon" src={format!("/icons/discipline/{}.svg", unit.discipline)} title={discipline_tooltip} />
            <div class="mental" title={format!("Morale: {}", unit.morale)}>
              <span class="morale">{ unit.morale }</span>
              if unit.stamina > 0 || unit.inexhaustible {
                <StaminaDetails unit={&unit} />
              }
            </div>
          </div>
          <TerrainDetails unit={&unit} />
          if let Some(ref weapon) = unit.primary_weapon {
            <WeaponRow class="weapon1-row" unit={&unit} {weapon} />
          }
          if let Some(ref weapon) = unit.secondary_weapon {
            <WeaponRow class="weapon2-row" unit={&unit} {weapon} />
          }
          <DefenseRow class="defense1-row" def={&unit.defense} hp={unit.hp} />
          if unit.has_mount {
            <DefenseRow class="defense2-row" mount={true} def={&unit.defense_mount} hp={unit.hp_mount} />
          }
          <div class="abilities">
            {for abilities}
          </div>
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(TerrainDetails)]
pub fn terrain_details(unit: Unit) -> Html {
    use std::fmt::Write as _;

    let mut title = "".to_string();
    if unit.ground_bonus.scrub != 0 {
        let _ = write!(title, "\n{:+}", unit.ground_bonus.scrub);
    }
    if unit.ground_bonus.forest != 0 {
        let _ = write!(title, "\n{:+}", unit.ground_bonus.forest);
    }
    if unit.ground_bonus.sand != 0 {
        let _ = write!(title, "\n{:+}", unit.ground_bonus.sand);
    }
    if unit.ground_bonus.snow != 0 {
        let _ = write!(title, "\n{:+}", unit.ground_bonus.snow);
    }
    if title.len() > 0 {
        title.remove(0);
    }

    html! {
      <div class="terrain" {title}>
        if unit.ground_bonus.scrub != 0 {
          <div class="scrub">
            if unit.ground_bonus.scrub > 0 {
              <img class="attribute" src="/icons/terrain/scrub-down.svg" />
            } else {
              <img class="attribute" src="/icons/terrain/scrub-up.svg" />
            }
            <span>{ format!("{:+}", unit.ground_bonus.scrub) }</span>
          </div>
        }
        if unit.ground_bonus.forest != 0 {
          <div class="forest">
            if unit.ground_bonus.forest > 0 {
              <img class="attribute" src="/icons/terrain/forest-down.svg" />
            } else {
              <img class="attribute" src="/icons/terrain/forest-up.svg" />
            }
            <span>{ format!("{:+}", unit.ground_bonus.forest) }</span>
          </div>
        }
        if unit.ground_bonus.sand != 0 {
          <div class="sand">
            if unit.ground_bonus.sand > 0 {
              <img class="attribute" src="/icons/terrain/sand-down.svg" />
            } else {
              <img class="attribute" src="/icons/terrain/sand-up.svg" />
            }
            <span>{ format!("{:+}", unit.ground_bonus.sand) }</span>
          </div>
        }
        if unit.ground_bonus.snow != 0 {
          <div class="snow">
            if unit.ground_bonus.snow > 0 {
              <img class="attribute" src="/icons/terrain/snow-down.svg" />
            } else {
              <img class="attribute" src="/icons/terrain/snow-up.svg" />
            }
            <span>{ format!("{:+}", unit.ground_bonus.snow) }</span>
          </div>
        }
      </div>
    }
}

#[autoprops]
#[function_component(StaminaDetails)]
pub fn stamina_details(unit: Unit) -> Html {
    use std::fmt::Write as _;

    let title = if unit.inexhaustible {
        "Inexhaustible".into()
    } else {
        let mut title = format!("{} stamina", unit.stamina);
        if unit.heat != 0 {
            let _ = write!(
                title,
                "\nHeat {}: {:+}",
                if unit.heat > 0 { "penalty" } else { "bonus" },
                -unit.heat,
            );
        }
        title
    };

    html! {
      <div class="stamina" {title}>
        if unit.stamina > 0 {
          <img src="/icons/attribute/stamina.svg" class="attribute" />
          <span>{ unit.stamina }</span>
          if unit.heat != 0 {
            <img src="/icons/attribute/heat.svg" class="attribute" />
            <span>{ format!("{:+}", -unit.heat) }</span>
          }
        } else {
          <img src="/icons/attribute/inexhaustible.svg" class="attribute" />
        }
      </div>
    }
}

#[autoprops]
#[function_component(WeaponRow)]
pub fn weapon_row(#[prop_or_default] class: AttrValue, unit: Unit, weapon: Weapon) -> Html {
    use std::fmt::Write as _;

    let (icon, mut title) = match weapon.class {
        WeaponType::Melee => ("/icons/weapon/blade.svg", "Melee weapon".to_string()),
        WeaponType::Spear => ("/icons/weapon/spear.svg", "Spear".into()),
        WeaponType::Missile => ("/icons/weapon/missile.svg", "Missile weapon".into()),
        WeaponType::Thrown => ("/icons/weapon/thrown.svg", "Thrown weapon".into()),
        WeaponType::Gunpowder => ("/icons/weapon/gunpowder.svg", "Gunpowder weapon".into()),
    };
    let lethality = format!("{}%", (weapon.lethality * 100.0).round());
    if weapon.lethality != 1.0 {
        let _ = write!(title, "\n    {} lethal", lethality);
    }
    let title: AttrValue = title.into();

    let strength = weapon.factor;
    let mut details = format!("Strength: {strength}");
    let extra = if weapon.is_missile {
        weapon.range
    } else {
        weapon.charge
    };
    if extra > 0 {
        let _ = write!(
            details,
            "\n{}: {extra}",
            if weapon.is_missile {
                "Range"
            } else {
                "Charge bonus"
            }
        );
    }
    if weapon.is_missile && !unit.infinite_ammo {
        let _ = write!(details, "\nAmmo: {}", weapon.ammo);
    }
    if weapon.spear_bonus > 0 {
        let _ = write!(details, "\nBonus against cavalry: {}", weapon.spear_bonus);
    }
    if weapon.armor_piercing {
        let _ = write!(details, "\nArmor piercing");
    }
    if weapon.pre_charge {
        let _ = write!(details, "\nThrown before charge");
    }
    let details: AttrValue = details.into();

    html! {
      <div {class}>
        <img class="icon" src={icon} title={&title} />
        if weapon.lethality != 1.0 {
          <div class="lethality" title={&title}>{ lethality }</div>
        }
        <div class="strength" title={&details}>{ strength }</div>
        <div class="details" title={&details}>
          if weapon.is_missile {
            <img src="/icons/attribute/range.svg" class="attribute" />
            <span>{ weapon.range }</span>
            if !unit.infinite_ammo {
              <img src="/icons/attribute/ammo.svg" class="attribute" />
              <span>{ weapon.ammo }</span>
            }
          } else if weapon.charge > 0 {
            <img src="/icons/attribute/charge.svg" class="attribute" />
            <span>{ weapon.charge }</span>
          }
          if weapon.spear_bonus > 0 {
            <img src="/icons/attribute/against-cavalry.svg" class="attribute" />
            <span>{ weapon.spear_bonus }</span>
          }
          if weapon.armor_piercing {
            <img src="/icons/attribute/armor-piercing.svg" class="attribute" />
          }
          if weapon.pre_charge {
            <img src="/icons/attribute/precharge.svg" class="attribute" />
          }
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(DefenseRow)]
pub fn defense_row(
    #[prop_or_default] class: AttrValue,
    #[prop_or_default] mount: bool,
    def: Defense,
    hp: u32,
) -> Html {
    use std::fmt::Write as _;

    let title = if mount { "Defense (mount)" } else { "Defense" };
    let icon = if mount {
        "/icons/stat/defense2.svg"
    } else {
        "/icons/stat/defense.svg"
    };

    let strength = def.total();
    let mut details = format!("Defense: {strength}");
    if def.armor > 0 {
        let _ = write!(details, "\n    Armor: {}", def.armor);
    }
    if def.skill > 0 {
        let _ = write!(details, "\n    Skill: {}", def.skill);
    }
    if def.shield > 0 {
        let _ = write!(details, "\n    Shield: {}", def.shield);
    }
    let details: AttrValue = details.into();

    html! {
      <>
        if strength > 0 || hp > 1 {
          <div {class}>
            <img class="icon" src={icon} {title} />
            <div class="strength" title={&details}>
              { if strength > 0 { strength } else { 0 } }
            </div>
            <div class="details" title={&details}>
              if def.armor > 0 {
                <img src="/icons/attribute/armor.svg" class="attribute" />
                <span>{ def.armor }</span>
              }
              if def.skill > 0 {
                <img src="/icons/attribute/skill.svg" class="attribute" />
                <span>{ def.skill }</span>
              }
              if def.shield > 0 {
                <img src="/icons/attribute/shield.svg" class="attribute" />
                <span>{ def.shield }</span>
              }
            </div>
            if hp > 1 {
              <div class="hp" title={format!("{hp} hit points")}>
                <img src="/icons/ability/heart.svg" />
                <span>{ hp }</span>
              </div>
            }
          </div>
        }
      </>
    }
}
