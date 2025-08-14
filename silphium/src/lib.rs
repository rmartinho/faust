#![feature(iter_intersperse)]

use crate::model::Module;
use crate::routes::switch;
use gloo::history::{AnyHistory, MemoryHistory};
use gloo::net::http::Request;
use implicit_clone::ImplicitClone;
use implicit_clone::unsync::{IArray, IString};
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
            Request::get("/mods.cbor")
                .send()
                .await
                .unwrap()
                .binary()
                .await
        })?;
        let modules: ModuleMap = ciborium::from_reader(res.as_ref().unwrap().as_slice()).unwrap();
        AppContext { modules }
    };

    Ok(html! {
      <ContextProvider<AppContext> {context}>
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
        <Footer />
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
#[function_component(Footer)]
fn footer() -> Html {
    html! {
      <footer>
        <span>{"Generated with "}<a href="https://faust.rmf.io">{"FAUST"}</a>{" with this "}<a href="/faust.yml">{"manifest"}</a>{"."}</span>
      </footer>
    }
}

#[autoprops]
#[function_component(StaticAppContent)]
fn static_app_content(route: &Route, data: IArray<u8>) -> Html {
    let modules: ModuleMap = ciborium::from_reader(data.as_slice()).unwrap();
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
        <Footer />
      </ContextProvider<AppContext>>
    }
}

#[autoprops(StaticAppProps)]
#[function_component(StaticApp)]
pub fn static_app(route: &Route, data: IArray<u8>) -> HtmlResult {
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
