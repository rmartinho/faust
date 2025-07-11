use crate::{
    AppContext,
    components::Link,
    modules::{Faction, Module},
    routes::Route,
};
use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::hooks::use_route;

#[autoprops]
#[function_component(ModulePage)]
pub fn module_page(id: IString) -> Html {
    let ctx = use_context::<AppContext>().expect("no context");
    let module = &ctx.modules[&id];

    let links = module
        .factions
        .values()
        .map(|f| html! {<FactionLink to={f}/>});
    let module = module.clone();

    html! {
      <div class="module-page">
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
      <Link to={route}>
        <img class="back" title="to mod list" src="/icons/ui/back.png" />
      </Link>
    }
}

#[autoprops]
#[function_component(FactionLink)]
fn faction_link(to: Faction) -> Html {
    let Some(Route::Module { module }) = use_route::<Route>() else {
        unreachable!()
    };

    let route = Route::Faction {
        module,
        faction: to.id_or_alias(),
    };
    html! {
      <Link to={route}>
        <img class="icon" src={to.image} title={&to.name} />
        <div class="name">{ to.name }</div>
      </Link>
    }
}

#[autoprops]
#[function_component(ModuleHeader)]
fn module_header(module: Module) -> Html {
    html! {
      <div class="banner">
        <img class="logo" src={module.logo} title={module.name} />
      </div>
    }
}
