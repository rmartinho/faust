use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::model::Unit;

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
#[function_component(UnitCard)]
pub fn unit_card(unit: Unit) -> Html {
    html! {
      <div class="unit-card">
        <img src={unit.image} />
        <span>{unit.name}</span>
      </div>
    }
}
