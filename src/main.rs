use crate::modules::Module;
use crate::routes::{Route, switch};
use indexmap::IndexMap;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

mod components;
mod modules;
mod routes;

const MODULES: &str = include_str!("../data/mods.json");

#[derive(Clone, PartialEq)]
pub struct Context {
    modules: Vec<Module>,
}

#[autoprops(AppProps)]
#[function_component(App)]
pub fn app(modules: &Vec<Module>) -> Html {
    html! {
      <ContextProvider<Context> context={Context{ modules: modules.clone()}}>
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </ContextProvider<Context>>
    }
}

fn main() {
    let modules: IndexMap<String, Module> = serde_json::from_str(MODULES).unwrap();
    yew::Renderer::<App>::with_props(AppProps {
        modules: modules.values().cloned().collect(),
    })
    .render();
}
