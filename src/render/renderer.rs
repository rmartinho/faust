use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

use askama::Template as _;
use image::{ImageError, ImageFormat, ImageReader, imageops::FilterType};
use implicit_clone::unsync::IString;
use silphium::{
    ModuleMap, Route, StaticApp, StaticAppProps,
    model::{Faction, Module, Unit},
};
use tokio::{fs, io};
use yew_router::Routable as _;

use crate::{
    args::Config,
    render::templates::{FILESYSTEM_STATIC, IndexHtml, RedirectHtml},
    utils::{read_file, write_file},
};

#[derive(Clone)]
pub struct Renderer {
    pub cfg: Config,
    pub routes: Vec<RenderRoute>,
    pub data: String,
    pub modules: ModuleMap,
}

impl Renderer {
    pub fn new(cfg: &Config, modules: ModuleMap) -> Self {
        let routes = collect_routes(&modules);
        Self {
            routes,
            cfg: cfg.clone(),
            data: String::new(),
            modules,
        }
    }

    pub async fn render(&mut self) -> io::Result<()> {
        self.create_directory().await?;
        self.create_static_files().await?;
        self.render_images().await?;
        self.render_data().await?;
        self.render_routes().await?;
        Ok(())
    }

    async fn render_images(&mut self) -> io::Result<()> {
        println!("Rendering images...");
        for m in self.modules.values_mut() {
            let src = self.cfg.src_dir.join(m.banner.as_ref());
            let banner_path = Self::module_banner_path(m);
            let dst = self.cfg.out_dir.join(&banner_path);
            println!("\t/{}", banner_path.display());
            Self::render_image(&src, &dst, MOD_BANNER_SIZE).await?;

            for f in m.factions.values_mut() {
                let src = self.cfg.src_dir.join(f.image.as_ref());
                let symbol_path = Self::faction_symbol_path(&m.id, f);
                let dst = self.cfg.out_dir.join(&symbol_path);
                println!("\t/{}", symbol_path.display());
                Self::render_image(&src, &dst, FACTION_SYMBOL_SIZE).await?;

                let mut roster: Vec<_> = f.roster.iter().collect();
                for u in roster.iter_mut() {
                    let src = self.cfg.src_dir.join(u.image.as_ref());
                    let portrait_path = Self::unit_portrait_path(&m.id, &f.id, u);
                    let dst = self.cfg.out_dir.join(&portrait_path);
                    println!("\t/{}", portrait_path.display());
                    Self::render_image(&src, &dst, UNIT_PORTRAIT_SIZE).await?;
                }
                f.roster = roster.into();
            }
        }
        Ok(())
    }

    fn module_banner_path(module: &mut Module) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module.id.as_ref())
            .join("banner.webp");
        module.banner = format!("/{}", path.display()).into();
        path
    }

    fn faction_symbol_path(module_id: &IString, faction: &mut Faction) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id.as_ref())
            .join("factions")
            .join(faction.id.as_ref())
            .with_extension("webp");
        faction.image = format!("/{}", path.display()).into();
        path
    }

    fn unit_portrait_path(module_id: &IString, faction_id: &IString, unit: &mut Unit) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id.as_ref())
            .join("units")
            .join(faction_id.as_ref())
            .join(unit.key.as_ref())
            .with_extension("webp");
        unit.image = format!("/{}", path.display()).into();
        path
    }

    async fn render_image(from: &Path, to: &Path, (height, width): (u32, u32)) -> io::Result<()> {
        let buf = read_file(from).await?;
        let format = ImageFormat::from_path(from).map_err(from_image_error)?;
        let img = ImageReader::with_format(Cursor::new(buf), format)
            .decode()
            .map_err(from_image_error)?;
        let img = img.resize(height, width, FilterType::Lanczos3);
        let mut buf = vec![];
        img.write_to(&mut Cursor::new(&mut buf), ImageFormat::WebP)
            .map_err(from_image_error)?;
        write_file(&to, buf).await?;
        Ok(())
    }

    async fn render_data(&mut self) -> io::Result<()> {
        println!("Rendering mod data...");
        let data = serde_json::to_string(&self.modules)?;
        self.data = data.clone();
        write_file(&self.cfg.out_dir.join("mods.json"), data).await?;
        Ok(())
    }

    async fn render_routes(&self) -> io::Result<()> {
        println!("Rendering routes...");
        for r in &self.routes {
            println!("\t{}", r.route.to_path());
            if let Some(ref target) = r.redirect {
                write_file(
                    &self.cfg.out_dir.join(&r.path),
                    RedirectHtml { target }.render()?,
                )
                .await?;
            } else {
                let body = &self.render_route(r.route.clone()).await;
                write_file(
                    &self.cfg.out_dir.join(&r.path),
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
        if self.cfg.out_dir.exists() {
            println!("Clearing output directory...");
            fs::remove_dir_all(&self.cfg.out_dir).await?;
        }
        println!("Creating output directory...");
        fs::create_dir_all(&self.cfg.out_dir).await
    }

    async fn create_static_files(&self) -> io::Result<()> {
        println!("Creating static files...");
        for file in FILESYSTEM_STATIC {
            file.create(&self.cfg.out_dir).await?;
        }
        Ok(())
    }
}

fn from_image_error(e: ImageError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
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

const MOD_BANNER_SIZE: (u32, u32) = (512, 256);
const FACTION_SYMBOL_SIZE: (u32, u32) = (128, 128);
const UNIT_PORTRAIT_SIZE: (u32, u32) = (82, 112);
