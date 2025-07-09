use crate::{
    AppContext,
    modules::{Faction, Module},
    routes::Route,
};
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::components::Link;

#[autoprops]
#[function_component(ModulePage)]
pub fn module_page(id: &AttrValue) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let module = ctx.modules[id].clone();

    let links = module.factions.values().map(|f| faction_link(&module, f));
    let module = module.clone();

    html! {
      <div class="module">
        <header>
          <BackLink />
          <ModuleHeader {module} />
        </header>
        <main>
          {for links}
        </main>
      </div>
    }
}

#[function_component(BackLink)]
fn back_link() -> Html {
    let route = Route::Home;
    html! {
      <Link<Route> to={route}>
        <img class="back" title="to mod list" src="icons/ui/back.png" />
      </Link<Route>>
    }
}

fn faction_link(module: &Module, faction: &Faction) -> Html {
    let f = faction.clone();
    let route = Route::Faction {
        module: module.id.clone(),
        faction: f.id_or_alias(),
    };
    html! {
      <Link<Route> to={route}>
        <img class="icon" src={f.image} title={f.name.clone()} />
        <div class="name">{ f.name }</div>
      </Link<Route>>
    }
}

#[autoprops]
#[function_component(ModuleHeader)]
fn module_header(module: &Module) -> Html {
    let m = module.clone();
    html! {
      <div class="banner">
        <img class="logo" src={m.logo} title={m.name} />
      </div>
    }
}
