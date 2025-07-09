use crate::{Context, routes::Route};
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(ModuleList)]
pub fn module_list() -> Html {
    let ctx = use_context::<Context>().expect("no context");

    let links = ctx.modules.iter().map(|m| {
        let m = m.clone();
        html! {
          <Link<Route> to={Route::Module { module: m.id }}>
            <img class="logo" src={ m.logo } title={ m.name } />
          </Link<Route>>
        }
    });

    html! {
      <div class="mods" data-mod-list="">
        {for links}
      </div>
    }
}
