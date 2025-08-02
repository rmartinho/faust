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
    Faction { module: IString, faction: IString },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn back(&self) -> Self {
        match self {
            Route::Home => Route::Home,
            Route::Module { .. } => Route::Home,
            Route::Faction { module, .. } => Route::Module {
                module: module.clone(),
            },
            _ => Route::Home,
        }
    }
}

// TODO bad module/faction/era IDs
pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <ModuleList /> },
        Route::Module { module } => html! { <ModulePage id={module} /> },
        Route::Faction { module, faction } => {
            html! { <FactionPage module_id={module} faction_id={faction} /> }
        }
        Route::NotFound => html! { <Redirect<Route> to={Route::Home} /> },
    }
}
