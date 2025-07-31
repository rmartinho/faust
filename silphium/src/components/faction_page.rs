use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

use crate::{
    AppContext,
    components::{BackLink, FactionRoster, Link, Text, UnitFilter},
    model::{Faction, Module},
    routes::Route,
};

#[autoprops]
#[function_component(FactionPage)]
pub fn faction_page(
    module_id: AttrValue,
    faction_id: AttrValue,
    #[prop_or_default] era: Option<AttrValue>,
) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let module = &ctx.modules[&module_id];
    let aliases = &module.aliases;
    let faction_id = aliases.get(&faction_id).unwrap_or(&faction_id);
    let faction = module.factions[faction_id].clone();

    if era.is_none() && faction.eras.len() > 1 {
        let route = Route::FactionEra {
            module: module.id.clone(),
            faction: faction.id_or_alias(),
            era: faction.eras[0].clone(),
        };
        return html! {
          <Redirect<Route> to={route}/>
        };
    }

    let filter_state = use_state(|| UnitFilter {
        era: era.or(faction.eras.get(0)),
        horde: if faction.is_horde { Some(false) } else { None },
    });
    let filter = &(*filter_state).clone();

    let toggle_horde = Callback::from(move |_| {
        filter_state.set(UnitFilter {
            horde: filter_state.horde.map(|h| !h),
            era: filter_state.era.clone(),
        })
    });

    html! {
    <div class="faction-page">
      <div class="header-container">
        <div class="nav">
          <BackLink />
          <button>
            <img class="settings button" title="Configure" src="/icons/ui/settings.webp" />
          </button>
          <button>
            <img class="help button" title="Help" src="/icons/ui/help.webp" />
          </button>
        </div>
        <FactionHeader classes={classes!("header")} {module} faction={faction.clone()} {filter} {toggle_horde} />
      </div>
      <FactionRoster roster={faction.roster} {filter} />
    // <template v-if="faction.id == 'mercs'">
    //   <MercenaryRoster :pools />
    // </template>
    // <template v-else>
    //   <FactionRoster :roster="faction.roster" />
    //   <AorRoster :roster="faction.roster" />
    // </template>
    </div>
    }
}

#[autoprops]
#[function_component(FactionHeader)]
pub fn faction_header(
    classes: Classes,
    module: Module,
    faction: Faction,
    #[prop_or_default] filter: UnitFilter,
    toggle_horde: Callback<()>,
) -> Html {
    let _ = module;
    let era_links = faction
        .eras
        .iter()
        .map(|e| html! {<EraLink to={&e} active={filter.era == Some(e)}/>});

    let onclick = Callback::from(move |_| toggle_horde.emit(()));

    html! {
      <div class={classes!("faction-header", classes)}>
        <div class="main">
          <div class="name"><Text text={faction.name} /></div>
            if faction.eras.len() > 1 {
              <div class="eras">
                {for era_links}
              </div>
            }
            if let Some(horde)= filter.horde {
              <div class="eras">
                <button {onclick}>
                  <div class={classes!("era", if horde {Some("checked")} else {None})}>
                    <svg title={if horde { "Show settled units" } else { "Show horde units" }}>
                      <use href={if horde { HORDE_ICON } else { HORDE_ICOFF }} />
                    </svg>
                  </div>
                </button>
              </div>
            }
          </div>
        <img class="icon" src={faction.image} />
      </div>
    }
}

#[autoprops]
#[function_component(EraLink)]
fn era_link(to: AttrValue, active: bool) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let era = to.clone();
    let (module_id, era_route) = match use_route::<Route>() {
        Some(Route::Faction { module, faction }) => (
            module.clone(),
            Route::FactionEra {
                module,
                faction,
                era,
            },
        ),
        Some(Route::FactionEra {
            module, faction, ..
        }) => (
            module.clone(),
            Route::FactionEra {
                module,
                faction,
                era,
            },
        ),
        _ => unreachable!(),
    };
    let module = &ctx.modules[&module_id];

    let era = module.eras[&to].clone();
    html! {
      <Link to={era_route}>
        <div class={classes!("era", if active {Some("checked")} else {None})}>
          <img src={if active { &era.icon } else { &era.icoff }} title={&era.name} />
          <span><Text text={era.name} /></span>
        </div>
      </Link>
    }
}

const HORDE_ICON: &str = "/icons/ui/horde.svg#on";
const HORDE_ICOFF: &str = "/icons/ui/horde.svg#off";
