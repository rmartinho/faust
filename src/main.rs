use std::path::{Path, PathBuf};

use askama::Template as _;
use silphium::{ModuleMap, Route, StaticApp, StaticAppProps};
use yew_router::Routable as _;

use crate::{
    templates::{FILESYSTEM_STATIC, IndexHtml, RedirectHtml},
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
        if let Some(ref target) = r.redirect {
            write_file(
                Path::join(&env.out_dir, &r.path),
                RedirectHtml { target }.render()?,
            )
            .await?;
        } else {
            let body = &render_route(r.route.clone(), env.clone()).await;
            write_file(
                Path::join(&env.out_dir, &r.path),
                IndexHtml { head: "", body }.render()?,
            )
            .await?;
        }
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
    pub redirect: Option<String>,
    pub path: PathBuf,
}

impl Env {
    async fn new() -> std::io::Result<Self> {
        let root_dir: PathBuf = ".".into();
        let out_dir = root_dir.clone().join("faust");

        if out_dir.exists() {
            tokio::fs::remove_dir_all(&out_dir).await?;
        }

        tokio::fs::create_dir_all(&out_dir).await?;

        const MODULES: &str = include_str!("../silphium/data/mods.json"); // HACK, insert parser here

        let mut routes = Vec::new();
        routes.push(RenderRoute {
            route: Route::Home,
            path: "index.html".into(),
            redirect: None,
        });
        routes.push(RenderRoute {
            route: Route::NotFound,
            path: "404.html".into(),
            redirect: None,
        });

        let modules: ModuleMap = serde_json::from_str(MODULES).unwrap();
        for module in modules.values() {
            routes.push(prepare_route(Route::Module {
                module: module.id.clone(),
            }));

            for faction in module.factions.values() {
                let id_or_alias = faction.id_or_alias();
                let route: Route = Route::Faction {
                    module: module.id.clone(),
                    faction: id_or_alias.clone(),
                };
                if id_or_alias != faction.id {
                    routes.push(prepare_redirect(
                        Route::Faction {
                            module: module.id.clone(),
                            faction: faction.id.clone(),
                        },
                        route.clone(),
                    ));
                    for era in faction.eras.iter() {
                        routes.push(prepare_redirect(
                            Route::FactionEra {
                                module: module.id.clone(),
                                faction: faction.id.clone(),
                                era: era.clone(),
                            },
                            Route::FactionEra {
                                module: module.id.clone(),
                                faction: id_or_alias.clone(),
                                era,
                            },
                        ));
                    }
                }
                if faction.eras.len() <= 1 {
                    if faction.eras.len() == 1 {
                        routes.push(prepare_redirect(
                            Route::FactionEra {
                                module: module.id.clone(),
                                faction: id_or_alias.clone(),
                                era: faction.eras[0].clone(),
                            },
                            route.clone(),
                        ))
                    }
                    routes.push(prepare_route(route));
                } else {
                    routes.push(prepare_redirect(
                        route,
                        Route::FactionEra {
                            module: module.id.clone(),
                            faction: faction.id_or_alias(),
                            era: faction.eras[0].clone(),
                        },
                    ))
                }

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
    RenderRoute {
        path,
        route,
        redirect: None,
    }
}

fn prepare_redirect(from: Route, to: Route) -> RenderRoute {
    let path = from.to_path();
    let path = PathBuf::from(&path[1..]).join("index.html");
    let target = to.to_path();
    RenderRoute {
        path,
        route: from,
        redirect: Some(target),
    }
}
