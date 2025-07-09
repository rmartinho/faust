use crate::modules::Module;
use crate::routes::{Route, switch};
use implicit_clone::ImplicitClone;
use indexmap::IndexMap;
use yew::prelude::*;
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

#[cfg(not(feature = "static"))]
mod wrapper {
    use gloo::net::http::Request;
    use yew::{prelude::*, suspense::use_future};

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
                .json::<super::ModuleMap>()
                .await
        })?;
        let Ok(ref modules) = *res else { todo!() };
        let context = super::AppContext {
            modules: modules.clone(),
        };

        Ok(html! { <super::App {context} /> })
    }
}

fn main() {
    yew::Renderer::<wrapper::Wrapper>::new().render();
}

type ModuleMap = IndexMap<AttrValue, Module>;
