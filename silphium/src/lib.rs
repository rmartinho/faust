#![feature(iter_intersperse)]

use crate::model::Module;
use crate::routes::switch;
use gloo::history::{AnyHistory, MemoryHistory};
use gloo::net::http::Request;
use implicit_clone::ImplicitClone;
use implicit_clone::unsync::IString;
use indexmap::IndexMap;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

mod components;
mod hooks;
pub mod model;
mod routes;

pub use routes::Route;
pub type ModuleMap = IndexMap<IString, Module>;

#[derive(ImplicitClone, Clone, PartialEq)]
struct AppContext {
    modules: ModuleMap,
}

#[function_component(AppContent)]
fn app_content() -> HtmlResult {
    let context = {
        let res = use_future(async || {
            Request::get("/mods.json")
                .send()
                .await
                .unwrap()
                .json::<ModuleMap>()
                .await
        })?;
        let Ok(ref modules) = *res else { todo!() };
        AppContext {
            modules: modules.clone(),
        }
    };

    Ok(html! {
      <ContextProvider<AppContext> {context}>
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </ContextProvider<AppContext>>
    })
}

#[function_component(App)]
fn app() -> HtmlResult {
    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
      <Suspense {fallback}>
        <AppContent />
      </Suspense>
    })
}

#[autoprops]
#[function_component(StaticAppContent)]
fn static_app_content(route: &Route, data: IString) -> Html {
    let modules: ModuleMap = serde_json::from_str(&data).unwrap();
    let context = AppContext { modules };

    let history: AnyHistory = {
        let path = route.to_path();
        let history = MemoryHistory::with_entries(vec![path]);
        history.into()
    };

    html! {
      <ContextProvider<AppContext> {context}>
        <Router {history}>
          <Switch<Route> render={switch} />
        </Router>
      </ContextProvider<AppContext>>
    }
}

#[autoprops(StaticAppProps)]
#[function_component(StaticApp)]
pub fn static_app(route: &Route, data: IString) -> HtmlResult {
    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
      <Suspense {fallback}>
        <StaticAppContent route={route.clone()} {data}/>
      </Suspense>
    })
}

#[wasm_bindgen(start)]
fn render() {
    #[cfg(not(feature = "hydration"))]
    {
        yew::Renderer::<App>::new().render();
    }
    #[cfg(feature = "hydration")]
    {
        yew::Renderer::<App>::new().hydrate();
    }
}
