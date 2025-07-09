use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::AppContext;

#[autoprops]
#[function_component(FactionPage)]
pub fn faction_page(module_id: &AttrValue, faction_id: &AttrValue) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let module = &ctx.modules[module_id];
    let aliases = &module.aliases;
    let faction_id = aliases.get(faction_id).unwrap_or(faction_id);
    let faction = module.factions[faction_id].clone();

    // let links = module.factions.values().map(|f| faction_link(&module, f));
    // let module = module.clone();

    html! {
      <div>{faction.name}</div>
    //   <div class="module">
    //     <header>
    //       <BackLink />
    //       <ModuleHeader {module} />
    //     </header>
    //     <main>
    //       {for links}
    //     </main>
    //   </div>
    }
}
