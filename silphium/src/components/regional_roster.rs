use std::collections::HashMap;

use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    components::{UnitCard, UnitFilter},
    model::{Aor, Faction},
};

#[autoprops]
#[function_component(RegionalRoster)]
pub fn regional_roster(faction: Faction, filter: UnitFilter) -> Html {
    let filter = &filter;
    let faction = &faction;
    let aors = faction.aors.into_iter().map(|aor| {
        html! {
          <AreaOfRecruitment {faction} {aor} {filter} />
        }
    });

    html! {
      <div class="roster">
        {for aors}
      </div>
    }
}

#[autoprops]
#[function_component(AreaOfRecruitment)]
pub fn area_of_recruitment(faction: Faction, aor: Aor, filter: UnitFilter) -> Html {
    let _ = filter; // TODO
    let units: HashMap<_, _> = faction.roster.iter().map(|u| (u.id.clone(), u)).collect();
    let cards: Vec<_> = aor
        .units
        .iter()
        .map(|u| {
            html! {
              <UnitCard unit={&units[&u]} />
            }
        })
        .collect();

    html! {
      <>
        if cards.len() > 0 {
          <div class="roster-group">
            <div class="map">
              if aor.name.len() > 0 {
                <span class="name">{aor.name}</span>
              }
              <img src={aor.map} />
            </div>
            <div class="unit-cards">
              {for cards}
            </div>
          </div>
        }
      </>
    }
}
