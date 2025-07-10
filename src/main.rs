use crate::modules::Module;
use crate::routes::{Route, switch};
use gloo::net::http::Request;
use implicit_clone::ImplicitClone;
use implicit_clone::unsync::IString;
use indexmap::IndexMap;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

mod components;
mod modules;
mod routes;

#[derive(ImplicitClone, Clone, PartialEq)]
pub struct AppContext {
    modules: ModuleMap,
}

#[autoprops]
#[function_component(App)]
pub fn app(context: AppContext) -> HtmlResult {
    Ok(html! {
      <ContextProvider<AppContext> {context}>
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </ContextProvider<AppContext>>
    })
}

#[function_component(Wrapper)]
pub fn wrapper() -> HtmlResult {
    let fallback = html! {<div>{"Loading..."}</div>};

    Ok(html! {
      <Suspense {fallback}>
        <Fetcher />
      </Suspense>
    })
}

#[function_component(Fetcher)]
pub fn fetcher() -> HtmlResult {
    let res = use_future(async || {
        Request::get("/mods.json")
            .send()
            .await
            .unwrap()
            .json::<ModuleMap>()
            .await
    })?;
    let Ok(ref modules) = *res else { todo!() };
    let context = AppContext {
        modules: modules.clone(),
    };

    Ok(html! { <App {context} /> })
}

fn main() {
    yew::Renderer::<Wrapper>::new().render();
}

type ModuleMap = IndexMap<IString, Module>;
