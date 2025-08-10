use std::cmp::max;

use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    components::{Icon, Text},
    model::{
        Ability, Defense, Discipline, Formation, MountType, PoolEntry, Unit, UnitClass, Weapon,
        WeaponType,
    },
};

fn pluralize<'a>(value: u32, singular: &'a str, plural: &'a str) -> &'a str {
    if value == 1 { singular } else { plural }
}

#[autoprops]
#[function_component(UnitCard)]
pub fn unit_card(unit: Unit, #[prop_or_default] pool: Option<PoolEntry>) -> Html {
    let unit = &unit;

    html! {
      <div class="unit-card">
        <div class="name row"><Text text={&unit.name} /></div>
        <div class="frame">
          <img class="image" title={&unit.name} src={&unit.image} />
          <SizeRow class="size-row row" {unit} />
          <CostRow class="cost-row row" {unit} />
          <UpkeepRow class="upkeep-row row" {unit} />
          <MentalRow class="mental-row row" {unit} />
          <TerrainRow class="terrain row" {unit} />
          <div class="weapons row">
            if let Some(ref weapon) = unit.primary_weapon {
              <WeaponRow class="weapon1-row" {unit} {weapon} />
            }
            if let Some(ref weapon) = unit.secondary_weapon {
              <WeaponRow class="weapon2-row" {unit} {weapon} />
            }
          </div>
          <div class="defenses row">
            <DefenseRow class="defense1-row" def={&unit.defense} hp={unit.hp} />
            if unit.mount.has_mount_stats() {
              <DefenseRow class="defense2-row" mount={true} def={&unit.defense_mount} hp={unit.hp_mount} />
            }
          </div>
          <AbilitiesRow class="abilities row" {unit} />
        </div>
        if let Some(pool) = pool {
          <PoolRow class="pool row" {pool} />
        }
      </div>
    }
}

#[autoprops]
#[function_component(SizeRow)]
pub fn size_row(#[prop_or_default] class: AttrValue, unit: Unit) -> Html {
    use std::fmt::Write as _;

    let mut soldiers_title = format!(
        "{} {}",
        unit.soldiers,
        pluralize(unit.soldiers, "soldier", "soldiers")
    );
    if unit.officers > 0 {
        let _ = write!(
            soldiers_title,
            "\n{} {}",
            unit.officers,
            pluralize(unit.officers, "officer", "officers")
        );
    }

    let formations = unit.formations.iter().map(|f| {
        let title = match &f {
            Formation::Square => "Square formation",
            Formation::Horde => "Horde",
            Formation::Phalanx => "Phalanx",
            Formation::Testudo => "Testudo",
            Formation::Wedge => "Wedge formation",
            Formation::Schiltrom => "Schiltrom",
            Formation::ShieldWall => "Shield wall",
        };
        html! {
          <Icon class="formation" {title} src="/icons/formation.svg" symbol={f.to_string()} />
        }
    });

    html! {
      <div {class}>
        <Icon class="icon" title="Soldiers" src="/icons/stat.svg" symbol="soldiers" />
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
    }
}

#[autoprops]
#[function_component(CostRow)]
pub fn cost_row(#[prop_or_default] class: AttrValue, unit: Unit) -> Html {
    html! {
      <div {class}>
        <Icon class="icon" title="Recruitment cost" src="/icons/stat.svg" symbol="cost" />
        <div class="cost" title={format!("Cost: {}", unit.cost)}>
          <span>{ unit.cost }</span>
          if unit.turns > 1 {
            <div class="turns" title={format!("{} turns", unit.turns)}>
              <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="turns" />
              <span>{ unit.turns }</span>
            </div>
          }
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(UpkeepRow)]
pub fn upkeep_row(#[prop_or_default] class: AttrValue, unit: Unit) -> Html {
    html! {
      <div {class}>
        <Icon class="icon" src="/icons/stat.svg"
            title={if unit.is_militia { "Upkeep cost (free in cities)" } else { "Upkeep cost"}}
            symbol={if unit.is_militia { "upkeep-castle" } else { "upkeep" }}
        />
        <div class="upkeep"
            title={format!("Upkeep{}: {}", if unit.is_militia { " (free in cities)" } else { "" }, unit.upkeep)}
        >
          <span>{ unit.upkeep }</span>
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(MentalRow)]
pub fn mental_row(#[prop_or_default] class: AttrValue, unit: Unit) -> Html {
    let discipline_tooltip = match unit.discipline {
        Discipline::Low => "Low discipline",
        Discipline::Normal => "Normal discipline",
        Discipline::Disciplined => "Disciplined",
        Discipline::Impetuous => "May charge without orders",
        Discipline::Berserker => "Berserker",
    };

    html! {
      <div {class}>
        <Icon class="icon" title={discipline_tooltip} src="/icons/discipline.svg" symbol={unit.discipline.to_string()} />
        <div class="mental" title={format!("Morale: {}", unit.morale)}>
          <span class="morale">{ unit.morale }</span>
          if unit.move_speed.is_some() || unit.stamina > 0 || unit.inexhaustible {
            <StaminaDetails unit={&unit} />
          }
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(TerrainRow)]
pub fn terrain_row(#[prop_or_default] class: AttrValue, unit: Unit) -> Html {
    use std::fmt::Write as _;

    let mut title = "".to_string();
    let mut write_details = |ground, v| {
        if v != 0 {
            let _ = writeln!(
                title,
                "{ground} {}: {v:+}",
                if v < 0 { "penalty" } else { "bonus" },
            );
        }
    };
    write_details("Scrub", unit.ground_bonus.scrub);
    write_details("Forest", unit.ground_bonus.forest);
    write_details("Sand", unit.ground_bonus.sand);
    write_details("Snow", unit.ground_bonus.snow);
    if title.len() > 0 {
        title.pop();
    }

    html! {
      <div {class} {title}>
        <GroundBonus class="scrub" ground="scrub" value={unit.ground_bonus.scrub} />
        <GroundBonus class="forest" ground="forest" value={unit.ground_bonus.forest} />
        <GroundBonus class="sand" ground="sand" value={unit.ground_bonus.sand} />
        <GroundBonus class="snow" ground="snow" value={unit.ground_bonus.snow} />
      </div>
    }
}

#[autoprops]
#[function_component(StaminaDetails)]
fn stamina_details(unit: Unit) -> Html {
    use std::fmt::Write as _;

    let speed = unit.move_speed.map(|s| {
        (
            s,
            if unit.mount.has_mount() {
                match s {
                    ..44 => 1,
                    ..51 => 2,
                    ..58 => 3,
                    _ => 4,
                }
            } else {
                match s {
                    ..28 => 1,
                    ..31 => 2,
                    ..34 => 3,
                    _ => 4,
                }
            },
        )
    });

    let mut title = if let Some((speed, _)) = &speed {
        format!("Speed: {speed}\n")
    } else {
        String::new()
    };
    if unit.inexhaustible {
        let _ = write!(title, "Inexhaustible");
    } else {
        let _ = write!(title, "Stamina: {}", unit.stamina);
        if unit.heat != 0 {
            let _ = write!(
                title,
                "\nHeat {}: {:+}",
                if unit.heat > 0 { "penalty" } else { "bonus" },
                -unit.heat,
            );
        }
    };

    html! {
      <div class="stamina" {title}>
        if let Some(speed) = speed {
          <Icon class="attribute" src="/icons/speed.svg" height={512} width={256} symbol={format!("speed-{}", speed.1)} />
          // <span>{ format!("{}", speed.0) }</span>
        }
        if unit.inexhaustible {
          <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="inexhaustible" />
        } else {
          if unit.stamina > 0 {
            <Icon class="attribute" height={512} width={256} src="/icons/attribute.svg" symbol="stamina" />
            <span>{ format!("{}", unit.stamina) }</span>
          }
          if unit.heat != 0 {
            <Icon class="attribute" height={512} width={256} src="/icons/attribute.svg" symbol="heat" />
            <span>{ format!("{:+}", -unit.heat) }</span>
          }
      }
      </div>
    }
}

#[autoprops]
#[function_component(WeaponRow)]
pub fn weapon_row(#[prop_or_default] class: AttrValue, unit: Unit, weapon: Weapon) -> Html {
    use std::fmt::Write as _;

    let mut title = match weapon.class {
        WeaponType::Melee => "Melee weapon".to_string(),
        WeaponType::Spear => "Spear".into(),
        WeaponType::Missile => "Missile weapon".into(),
        WeaponType::Thrown => "Thrown weapon".into(),
        WeaponType::Gunpowder if unit.class == UnitClass::Artillery => "Cannon".into(),
        WeaponType::Gunpowder => "Firearm".into(),
    };
    let lethality = format!("{}%", (weapon.lethality * 100.0).round());
    if weapon.lethality != 1.0 {
        let _ = write!(title, "\n    {} lethal", lethality);
    }
    let strength = weapon.factor;
    let _ = write!(title, "\n    Strength: {strength}");
    let extra = if weapon.is_missile {
        weapon.range
    } else {
        weapon.charge
    };
    if extra > 0 {
        let _ = write!(
            title,
            "\n    {}: {extra}",
            if weapon.is_missile {
                "Range"
            } else {
                "Charge bonus"
            }
        );
    }
    if weapon.is_missile && !unit.infinite_ammo {
        let _ = write!(title, "\n    Ammo: {}", weapon.ammo);
    }
    if weapon.spear_bonus > 0 {
        let _ = write!(title, "\n    Bonus against cavalry: {}", weapon.spear_bonus);
    }
    if weapon.armor_piercing {
        let _ = write!(title, "\n    Armor piercing");
    }
    if weapon.pre_charge {
        let _ = write!(title, "\n    Thrown before charge");
    }
    let title: AttrValue = title.into();

    let weapon_symbol = match (weapon.class, unit.class) {
        (WeaponType::Gunpowder, UnitClass::Artillery) => "cannon".into(),
        _ => weapon.class.to_string(),
    };

    html! {
      <div {class} {title}>
        <Icon class="icon" src="/icons/weapon.svg" symbol={weapon_symbol} />
        if weapon.lethality != 1.0 {
          <div class="lethality">{ lethality }</div>
        }
        <div class="strength">{ strength }</div>
        <div class="details">
          if weapon.is_missile {
            <Icon class="attribute" height={512} width={384} src="/icons/attribute.svg" symbol="range" />
            <span>{ weapon.range }</span>
            if !unit.infinite_ammo {
              <Icon class="attribute" height={512} width={256} src="/icons/attribute.svg" symbol="ammo" />
              <span>{ weapon.ammo }</span>
            }
          } else if weapon.charge > 0 {
            <Icon class="attribute" height={512} width={384} src="/icons/attribute.svg" symbol="charge" />
            <span>{ weapon.charge }</span>
          }
          if weapon.spear_bonus > 0 {
            <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="against-cavalry" />
            <span>{ weapon.spear_bonus }</span>
          }
          if weapon.armor_piercing {
            <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="armor-piercing" />
          }
          if weapon.pre_charge {
            <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="precharge" />
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

    let symbol = if mount { "defense-mount" } else { "defense" };

    let strength = def.total();
    let mut title = format!("Defense{}: {strength}", if mount { " (mount)" } else { "" });
    if def.armor > 0 {
        let _ = write!(title, "\n    Armor: {}", def.armor);
    }
    if def.skill > 0 {
        let _ = write!(title, "\n    Skill: {}", def.skill);
    }
    if def.shield > 0 {
        let _ = write!(title, "\n    Shield: {}", def.shield);
    }
    let title: AttrValue = title.into();

    html! {
      <>
        if strength > 0 || hp > 1 {
          <div {class} {title}>
            <Icon class="icon" src="/icons/stat.svg" {symbol} />
            <div class="strength">
              { if strength > 0 { strength } else { 0 } }
            </div>
            <div class="details">
              if def.armor > 0 {
                <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="armor" />
                <span>{ def.armor }</span>
              }
              if def.skill > 0 {
                <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="skill" />
                <span>{ def.skill }</span>
              }
              if def.shield > 0 {
                <Icon class="attribute" height={512} width={512} src="/icons/attribute.svg" symbol="shield" />
                <span>{ def.shield }</span>
              }
            </div>
            if hp > 1 {
              <div class="hp" title={format!("{hp} hit points")}>
                <Icon class="ability" src="/icons/ability.svg" symbol="heart" />
                <span>{ hp }</span>
              </div>
            }
          </div>
        }
      </>
    }
}

#[autoprops]
#[function_component(GroundBonus)]
fn ground_bonus(#[prop_or_default] class: AttrValue, ground: AttrValue, value: i32) -> Html {
    let up_or_down = if value > 0 { "up" } else { "down" };
    html! {
      <>
        if value != 0 {
          <div {class}>
            <Icon class="attribute" src="/icons/terrain.svg" symbol={format!("{ground}-{up_or_down}")} />
            <span>{ format!("{value:+}") }</span>
          </div>
        }
      </>
    }
}

#[autoprops]
#[function_component(AbilitiesRow)]
pub fn abilities_row(#[prop_or_default] class: AttrValue, unit: Unit) -> Html {
    let base = if unit.class == UnitClass::Ship {
        [html! {
          <Icon class="ability" title="Ship" src="/icons/class.svg" symbol="ship" />
        }]
    } else if unit.class == UnitClass::Artillery {
        [html! {
          <Icon class="ability" title="Artillery" src="/icons/class.svg" symbol="artillery" />
        }]
    } else if unit.class == UnitClass::General {
        [html! {
          <Icon class="ability" title="General" src="/icons/class.svg" symbol="general" />
        }]
    } else {
        [html! { <></> }]
    }
    .into_iter();
    let mount = if unit.mount == MountType::Horse {
        [html! {
          <Icon class="ability" title="Horse" src="/icons/mount.svg" symbol="horse" />
        }]
    } else if unit.mount == MountType::Camel {
        [html! {
          <Icon class="ability" title="Camel" src="/icons/mount.svg" symbol="camel" />
        }]
    } else if unit.mount == MountType::Elephant {
        [html! {
          <Icon class="ability" title="Elephant" src="/icons/mount.svg" symbol="elephant" />
        }]
    } else if unit.mount == MountType::Chariot {
        [html! {
          <Icon class="ability" title="Chariot" src="/icons/mount.svg" symbol="chariot" />
        }]
    } else {
        [html! { <></> }]
    }
    .into_iter();
    let abilities = base.chain(mount).chain(unit.abilities.iter().map(|ab| {
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
            Ability::Knight => "Receives knightly bonuses",
        };
        html! {
          <Icon class="ability" {title} src="/icons/ability.svg" symbol={ab.to_string()} />
        }
    }));

    html! {
      <div {class}>
        {for abilities}
      </div>
    }
}

#[autoprops]
#[function_component(PoolRow)]
pub fn pool_row(#[prop_or_default] class: AttrValue, pool: PoolEntry) -> Html {
    let p05 = pool.replenish.p05();
    let p50 = pool.replenish.p50();
    let p95 = pool.replenish.p95();
    let details: AttrValue = format!("Units replenish:\nin {p05} turns (5% of the time)\nin {p50} turns (50% of the time)\nin {p95} turns (95% of the time)").into();
    let range = max(p95 - p50, p50 - p05);
    let max = pool.max;
    let exp = pool.exp;

    html! {
      <div {class}>
        <div class="turns" title={details}>
          <Icon class="icon" src="/icons/attribute.svg" symbol="turns" />
          <div class="average">{ p50 }</div>
          <div class="interval">{ format!("±{range}") }</div>
        </div>
        <div class="max" title={format!("Max: {max} units")} >{ format!("×{max}") }</div>
        if pool.exp > 0 {
          <Icon class="exp" title={format!("{exp} experience")} src="/icons/exp.svg" symbol={format!("exp-{exp}")} />
        }
        // TODO restricts
        // <template v-if="faction">
        //   <img class="faction" :src="faction" :title="`${factionName} only`" />
        // </template>
      </div>
    }
}
