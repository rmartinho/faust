use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::{
    AppContext,
    components::{BackLink, Button, FactionRoster, RosterFilter, Text, UnitFilter},
    hooks::ModelHandle,
    model::{Faction, Module},
};

#[autoprops]
#[function_component(FactionPage)]
pub fn faction_page(module_id: AttrValue, faction_id: AttrValue) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let module = &ctx.modules[&module_id];
    let aliases = &module.aliases;
    let faction_id = aliases.get(&faction_id).unwrap_or(&faction_id);
    let faction = module.factions.get(faction_id).unwrap();

    let filter = use_state(|| UnitFilter {
        era: (faction.eras.len() > 1).then(|| faction.eras[0].clone()),
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
        <FactionRoster roster={&faction.roster} filter={&*filter} />
      </main>
    </div>
    }
}

#[autoprops]
#[function_component(FactionHeader)]
fn faction_header(
    class: Classes,
    module: Module,
    faction: Faction,
    filter: ModelHandle<UnitFilter>,
) -> Html {
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
