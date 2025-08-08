use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{components::{UnitCard, UnitFilter}, model::Pool};

#[autoprops]
#[function_component(MercenaryRoster)]
pub fn mercenary_roster(pools: IArray<Pool>, filter: UnitFilter) -> Html {
    let filter = &filter;
    let pools = pools.into_iter().map(|pool| {
        html! {
          <MercenaryPool {pool} {filter} />
        }
    });

    html! {
      <div class="roster">
        {for pools}
      </div>
    }
}

#[autoprops]
#[function_component(MercenaryPool)]
pub fn mercenary_pool(pool: Pool, filter: UnitFilter) -> Html {
    let _ = filter; // TODO
    let cards: Vec<_> = pool.units
        .iter()
        .map(|u| {
            html! {
              <UnitCard unit={&u.unit} pool={u}/>
            }
        })
        .collect();
    html! {
      <>
        if cards.len() > 0 {
          <div class="roster-group">
            <img class="map" src={pool.map} />
            <div class="unit-cards">
              {for cards}
            </div>
          </div>
        }
      </>
    }
}
