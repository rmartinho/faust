use crate::{AppContext, modules::Module, routes::Route};
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(ModuleList)]
pub fn module_list() -> Html {
    let ctx = use_context::<AppContext>().expect("no context");

    let links = ctx.modules.values().cloned().map(module_link);

    html! {
      <main>
        <div class="module-list">
          {for links}
        </div>
      </main>
    }
}

fn module_link(m: Module) -> Html {
    let route = Route::Module { module: m.id };
    html! {
      <Link<Route> to={route}>
        <img class="logo" src={ m.logo } title={ m.name } />
      </Link<Route>>
    }
}
