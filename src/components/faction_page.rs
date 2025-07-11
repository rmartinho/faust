use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

use crate::{
    AppContext,
    components::{BackLink, Link},
    modules::{Faction, Module},
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

    html! {
    <div class="faction-page">
      <div class="header-container">
        <div class="nav">
          <BackLink />
          <button>
            <img class="settings button" title="Configure" src="/icons/ui/settings.png" />
          </button>
          <button >
            <img class="help button" title="Help" src="/icons/ui/help.png" />
          </button>
        </div>
        <FactionHeader classes={classes!("header")} {module} {faction} {era} />
      </div>
    // <FactionRoster roster={faction.roster} />
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
    #[prop_or_default] era: Option<AttrValue>,
) -> Html {
    if era.is_none() && faction.eras.len() > 1 {
        let route = Route::FactionEra {
            module: module.id.clone(),
            faction: faction.id.clone(),
            era: faction.eras[0].clone(),
        };
        return html! {
          <Redirect<Route> to={route}/>
        };
    }

    let era_links = faction
        .eras
        .iter()
        .map(|e| html! {<EraLink to={&e} active={era == Some(e)}/>});

    html! {
      <div class={classes!("faction-header", classes)}>
        <div class="main">
          <div class="name">{ faction.name }</div>
            if faction.eras.len() > 1 {
              <div class="eras">
                {for era_links}
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
          <img src={&era.icon} title={&era.name} />
          <span>{ era.name }</span>
        </div>
      </Link>
    }
}
