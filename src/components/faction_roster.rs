use implicit_clone::unsync::{IArray, IString};
use indexmap::IndexMap;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::modules::Unit;

#[autoprops]
#[function_component(FactionRoster)]
pub fn faction_roster(roster: &IndexMap<IString, IArray<Unit>>) -> Html {
    let groups = roster.iter().map(|(group, units)| {
        html! {
          if units.len() > 0 {
            <RosterGroup {group} {units} />
          }
        }
    });

    html! {
      <div class="roster">
        {for groups}
      </div>
    }
}

#[autoprops]
#[function_component(RosterGroup)]
pub fn roster_group(group: IString, units: IArray<Unit>) -> Html {
    let cards = units.iter().map(|unit| {
        html! {
          <UnitCard {unit}/>
        }
    });

    html! {
      <div class="roster-group">
        <div class="legend">
          <img class="group" src={format!("/icons/class/{group}.svg")} />
        </div>
        <div class="unit-cards">
          {for cards}
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(UnitCard)]
pub fn unit_card(unit: Unit) -> Html {
    html! {
      <div class="unit-card">
        <img src={unit.image} />
        <span>{unit.name}</span>
      </div>
    }
}
