use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::model::{Discipline, Formation, Unit};

#[autoprops]
#[function_component(FactionRoster)]
pub fn faction_roster(roster: IArray<Unit>) -> Html {
    let cards = roster.iter().map(|unit| {
        html! {
          <UnitCard {unit}/>
        }
    });

    html! {
      <div class="roster">
        <div class="roster-group">
          <div class="legend">
            <img class="group" src="/icons/class/swords.svg" />
          </div>
          <div class="unit-cards">
            {for cards}
          </div>
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(StaminaDetails)]
pub fn stamina_details(unit: Unit) -> Html {
    use std::fmt::Write as _;

    let stamina = unit.stamina();
    let mut title = if stamina == u32::MAX {
        "Inexhaustible".into()
    } else {
        format!("{} stamina", stamina)
    };
    if unit.heat != 0 {
        let _ = write!(
            title,
            "\nHeat {}: {:+}",
            if unit.heat > 0 { "penalty" } else { "bonus" },
            -unit.heat,
        );
    }

    html! {
      <div class="stamina" {title}>
        if stamina > 0 {
          <img src="/icons/attribute/stamina.svg" class="attribute" />
          <span>{ stamina }</span>
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

    html! {
      <div class="unit-card">
        <div class="name">{ unit.name.clone() }</div>
        <div class="frame">
          <img class="image" title={&unit.name} src={&unit.image} />
          <div class="size-row">
            <img class="icon" src="/icons/stat/soldiers.svg" title="Soldiers"            />
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
              if unit.stamina() > 0 {
                <StaminaDetails unit={&unit} />
              }
            </div>
          </div>
          <TerrainDetails unit={&unit} />
        </div>
      </div>
    }
}
