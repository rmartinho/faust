use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{FactionPage, ModuleList, ModulePage};

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

// TODO bad module/faction/era IDs
pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <ModuleList /> },
        Route::Module { module } => html! { <ModulePage id={module} /> },
        Route::Faction { module, faction } => {
            html! { <FactionPage module_id={module} faction_id={faction} /> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        _ => html! { <h1>{ "TODO" }</h1> },
    }
}
