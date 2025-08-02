use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

use crate::{
    AppContext,
    components::{BackLink, Button, FactionRoster, Link, RosterFilter, Text, UnitFilter},
    hooks::ModelHandle,
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

    let filter = use_state(|| UnitFilter {
        era: era.or(faction.eras.get(0)),
        horde: faction.is_horde.then_some(false),
    });

    html! {
    <div class="faction-page">
      <header class="header-container">
        <div class="nav">
          <BackLink />
          <Button>
            <img class="settings button" title="Configure" src="/icons/ui/settings.webp" />
          </Button>
          <Button>
            <img class="help button" title="Help" src="/icons/ui/help.webp" />
          </Button>
        </div>
        <FactionHeader class="header" {module} faction={faction.clone()} filter={&filter} />
      </header>
      <main>
        <FactionRoster roster={faction.roster} filter={&*filter} />
      </main>
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
    class: Classes,
    module: Module,
    faction: Faction,
    filter: ModelHandle<UnitFilter>,
) -> Html {
    // let era_links = faction
    //     .eras
    //     .iter()
    //     .map(|e| html! {<EraLink to={&e} active={filter.era == Some(e)}/>});

    let faction = &faction;
    html! {
      <div class={classes!("faction-header", class)}>
        <div class="title">
          <div class="name"><Text text={&faction.name} /></div>
          <RosterFilter {module} {faction} {filter} />
        </div>
        <img class="icon" src={&faction.image} />
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
