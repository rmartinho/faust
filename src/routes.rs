use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{FactionPage, ModuleList, ModulePage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:module")]
    Module { module: IString },
    #[at("/:module/:faction")]
    Faction {
        module: IString,
        faction: IString,
    },
    #[at("/:module/:faction/:era")]
    FactionEra {
        module: IString,
        faction: IString,
        era: IString,
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
        Route::FactionEra { module, faction, era } => {
            html! { <FactionPage module_id={module} faction_id={faction} {era} /> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
