use implicit_clone::{ImplicitClone, unsync::IArray};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    components::{Icon, UnitCard},
    model::{Unit, UnitClass},
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
            <Icon class="legend" {title} src="/icons/class.svg" symbol={group.to_string()} />
            <div class="unit-cards">
              {for cards}
            </div>
          </div>
        }
      </>
    }
}
