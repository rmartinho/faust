use std::collections::HashMap;

use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    components::{UnitCard, UnitFilter},
    model::{Aor, Faction},
};

#[autoprops]
#[function_component(RegionalRoster)]
pub fn regional_roster(faction: Faction, aors: IArray<Aor>, filter: UnitFilter) -> Html {
    let filter = &filter;
    let aors = aors.into_iter().filter_map(move |aor| {
        let faction = &faction;
        (aor.faction == faction.id).then(move || {
            html! {
              <AreaOfRecruitment {faction} {aor} {filter} />
            }
        })
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
            <img class="map" src={aor.map} />
            <div class="unit-cards">
              {for cards}
            </div>
          </div>
        }
      </>
    }
}
