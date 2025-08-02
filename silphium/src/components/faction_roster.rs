use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    components::{Icon, UnitCard, UnitFilter},
    model::{Unit, UnitClass},
};

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
