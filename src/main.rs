use silphium::{Route, StaticApp, StaticAppProps};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MODULES: &str = include_str!("../silphium/data/mods.json");

    let res = yew::ServerRenderer::<StaticApp>::with_props(|| StaticAppProps {
        route: Route::Home,
        data: MODULES.into(),
    })
    .render()
    .await;
    println!("{res}");

    Ok(())
}
