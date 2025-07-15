use std::path::PathBuf;

use askama::Template as _;
use silphium::{ModuleMap, Route, StaticApp, StaticAppProps};
use tokio::{fs, io};
use yew_router::Routable as _;

use crate::{
    args::Args,
    render::templates::{FILESYSTEM_STATIC, IndexHtml, RedirectHtml},
    utils::write_file,
};

#[derive(Clone)]
pub struct Renderer {
    pub out_dir: PathBuf,
    pub routes: Vec<RenderRoute>,
    pub data: String,
}

impl Renderer {
    pub fn new(args: &Args, modules: ModuleMap) -> Self {
        let routes = collect_routes(&modules);
        Self {
            routes,
            out_dir: args.out_dir.clone().unwrap_or_else(|| "faust".into()),
            data: serde_json::to_string(&modules).unwrap(),
        }
    }

    pub async fn render(&self) -> io::Result<()> {
        self.create_directory().await?;
        self.create_static_files().await?;
        self.render_routes().await?;
        Ok(())
    }

    async fn render_routes(&self) -> io::Result<()> {
        for r in &self.routes {
            println!("{}", r.route.to_path());
            if let Some(ref target) = r.redirect {
                write_file(
                    &self.out_dir.join(&r.path),
                    RedirectHtml { target }.render()?,
                )
                .await?;
            } else {
                let body = &self.render_route(r.route.clone()).await;
                write_file(
                    &self.out_dir.join(&r.path),
                    IndexHtml { head: "", body }.render()?,
                )
                .await?;
            }
        }
        Ok(())
    }

    async fn render_route(&self, route: Route) -> String {
        let props = StaticAppProps {
            route,
            data: self.data.clone().into(),
        };
        let renderer = yew::LocalServerRenderer::<StaticApp>::with_props(props);
        renderer.render().await
    }

    async fn create_directory(&self) -> io::Result<()> {
        if self.out_dir.exists() {
            fs::remove_dir_all(&self.out_dir).await?;
        }
        fs::create_dir_all(&self.out_dir).await
    }

    async fn create_static_files(&self) -> io::Result<()> {
        for file in FILESYSTEM_STATIC {
            file.create(&self.out_dir).await?;
        }
        Ok(())
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
