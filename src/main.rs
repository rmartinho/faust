use silphium::{Route, StaticApp, StaticAppProps};

use crate::templates::IndexHtml;

mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MODULES: &str = include_str!("../silphium/data/mods.json");

    let body = &yew::ServerRenderer::<StaticApp>::with_props(|| StaticAppProps {
        route: Route::Home,
        data: MODULES.into(),
    })
    .render()
    .await;
    println!("{}", IndexHtml { head: "", body });

    Ok(())
}
