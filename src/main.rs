use std::path::{Path, PathBuf};

use askama::Template as _;
use silphium::{ModuleMap, Route, StaticApp, StaticAppProps};
use yew_router::Routable as _;

use crate::{
    templates::{FILESYSTEM_STATIC, IndexHtml},
    utils::write_file,
};

mod templates;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::new().await?;

    for file in FILESYSTEM_STATIC {
        file.create(&env.out_dir).await?;
    }

    render_routes(env).await?;
    Ok(())
}

async fn render_routes(env: Env) -> tokio::io::Result<()> {
    for r in &env.routes {
        let body = &render_route(r.route.clone(), env.clone()).await;
        write_file(
            Path::join(&env.out_dir, &r.path),
            IndexHtml { head: "", body }.render()?,
        )
        .await?;
    }
    Ok(())
}

async fn render_route(route: Route, env: Env) -> String {
    let props = StaticAppProps {
        route,
        data: env.data.into(),
    };
    let renderer = yew::LocalServerRenderer::<StaticApp>::with_props(props);
    renderer.render().await
}

#[derive(Clone)]
struct Env {
    _manifest_path: PathBuf,
    routes: Vec<RenderRoute>,
    out_dir: PathBuf,
    data: String,
}

#[derive(Clone)]
struct RenderRoute {
    pub route: Route,
    pub path: PathBuf,
}

impl Env {
    async fn new() -> std::io::Result<Self> {
        let root_dir: PathBuf = ".".into();
        let out_dir = root_dir.clone().join("faust");

        if out_dir.exists() {
            println!("Removing existing output directory");
            tokio::fs::remove_dir_all(&out_dir).await?;
        }

        println!("Creating output directory");
        tokio::fs::create_dir_all(&out_dir).await?;

        const MODULES: &str = include_str!("../silphium/data/mods.json"); // HACK, insert parser here

        let mut routes = Vec::new();
        routes.push(RenderRoute {
            route: Route::Home,
            path: "index.html".into(),
        });
        routes.push(RenderRoute {
            route: Route::NotFound,
            path: "404.html".into(),
        });

        let modules: ModuleMap = serde_json::from_str(MODULES).unwrap();
        for module in modules.values() {
            routes.push(prepare_route(Route::Module {
                module: module.id.clone(),
            }));

            for faction in module.factions.values() {
                routes.push(prepare_route(Route::Faction {
                    module: module.id.clone(),
                    faction: faction.id_or_alias(),
                }));

                for era in faction.eras.iter() {
                    routes.push(prepare_route(Route::FactionEra {
                        module: module.id.clone(),
                        faction: faction.id_or_alias(),
                        era,
                    }));
                }
            }
        }

        Ok(Self {
            _manifest_path: "faust.yml".into(),
            routes,
            out_dir,
            data: MODULES.into(),
        })
    }
}

fn prepare_route(route: Route) -> RenderRoute {
    let path: String = route.to_path();
    let path = PathBuf::from(&path[1..]).join("index.html");
    RenderRoute { path, route }
}
