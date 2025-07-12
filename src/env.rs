use std::path::PathBuf;

use silphium::{ModuleMap, Route};
use yew_router::Routable as _;

#[derive(Clone)]
pub struct Env {
    pub manifest_path: PathBuf,
    pub routes: Vec<RenderRoute>,
    pub out_dir: PathBuf,
    pub data: String,
}

impl Env {
    pub fn new() -> Self {
        const MODULES: &str = "[]";//include_str!("../silphium/data/mods.json"); // HACK, PARSER GOES HERE
        let modules = serde_json::from_str(MODULES).unwrap();

        let routes = collect_routes(&modules);
        Self {
            manifest_path: "faust.yml".into(),
            routes,
            out_dir: "faust".into(),
            data: MODULES.into(),
        }
    }
}

#[derive(Clone)]
pub struct RenderRoute {
    pub route: Route,
    pub redirect: Option<String>,
    pub path: PathBuf,
}

fn collect_routes(modules: &ModuleMap) -> Vec<RenderRoute> {
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
    routes
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
