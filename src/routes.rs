use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::module_list::ModuleList;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:module")]
    Module { module: AttrValue },
    #[at("/:module/:faction")]
    Faction {
        module: AttrValue,
        faction: AttrValue,
    },
    #[at("/:module/:faction/:era")]
    FactionEra {
        module: AttrValue,
        faction: AttrValue,
        era: AttrValue,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <ModuleList /> },
        Route::Module { module } => html! { <h1>{ module }</h1>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        _ => todo!(),
    }
}
