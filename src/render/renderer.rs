use std::{
    fmt::Write as _,
    io::Cursor,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context as _, Result};
use askama::Template as _;
use image::{ImageError, ImageFormat, ImageReader, imageops::FilterType};
use implicit_clone::unsync::IString;
use indicatif::{HumanBytes, MultiProgress, ProgressBar};
use silphium::{
    ModuleMap, Route, StaticApp, StaticAppProps,
    model::{Faction, Module, Unit},
};
use tokio::{fs, io};
use yew_router::Routable as _;

use crate::{
    args::Config,
    render::templates::{FILESYSTEM_STATIC, IndexHtml, RedirectHtml},
    utils::{FOLDER, LINK, PAPER, PICTURE, path_fallback, progress_style, read_file, write_file},
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

    pub async fn render(&mut self) -> Result<()> {
        let m = MultiProgress::new();
        self.create_directory(m.clone()).await?;
        self.create_static_files(m.clone()).await?;
        self.render_images(m.clone()).await?;
        self.render_data(m.clone()).await?;
        self.render_routes(m.clone()).await?;
        let _ = m.clear();
        Ok(())
    }

    async fn render_images(&mut self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[3/5]");
        pb.tick();
        pb.set_message(format!("{PICTURE}rendering images"));
        for m in self.modules.values_mut() {
            let src = path_fallback(&self.cfg, m.banner.as_ref(), None);
            let banner_path = Self::module_banner_path(m);
            let dst = self.cfg.out_dir.join(&banner_path);
            pb.tick();
            pb.set_message(format!("{PICTURE}rendering {}", web_path(&banner_path)));
            Self::render_image(&src, &dst, MOD_BANNER_SIZE).await?;

            for f in m.factions.values_mut() {
                let src = path_fallback(
                    &self.cfg,
                    f.image.as_ref(),
                    Some("data/ui/faction_icons/slave_blank.tga"),
                );
                let symbol_path = Self::faction_symbol_path(&m.id, f);
                let dst = self.cfg.out_dir.join(&symbol_path);
                pb.tick();
                pb.set_message(format!("{PICTURE}rendering {}", web_path(&symbol_path)));
                Self::render_image(&src, &dst, FACTION_SYMBOL_SIZE).await?;

                let mut roster: Vec<_> = f.roster.iter().collect();
                for u in roster.iter_mut() {
                    let src = path_fallback(
                        &self.cfg,
                        u.image.as_ref(),
                        Some("data/generic/generic_unit_card.tga"),
                    );
                    let portrait_path = Self::unit_portrait_path(&m.id, &f.id, u);
                    let dst = self.cfg.out_dir.join(&portrait_path);
                    pb.tick();
                    pb.set_message(format!("{PICTURE}rendering {}", web_path(&portrait_path)));
                    Self::render_image(&src, &dst, UNIT_PORTRAIT_SIZE).await?;
                }
                f.roster = roster.into();
            }
        }
        pb.finish_with_message(format!("{PICTURE}rendered images"));
        Ok(())
    }

    fn module_banner_path(module: &mut Module) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module.id.as_ref())
            .join("banner.webp");
        module.banner = web_path(&path).into();
        path
    }

    fn faction_symbol_path(module_id: &IString, faction: &mut Faction) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id.as_ref())
            .join("factions")
            .join(faction.id.as_ref())
            .with_extension("webp");
        faction.image = web_path(&path).into();
        path
    }

    fn unit_portrait_path(module_id: &IString, faction_id: &IString, unit: &mut Unit) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id.as_ref())
            .join("units")
            .join(faction_id.as_ref())
            .join(unit.key.as_ref())
            .with_extension("webp");
        unit.image = web_path(&path).into();
        path
    }

    async fn render_image(from: &Path, to: &Path, (height, width): (u32, u32)) -> Result<()> {
        let buf = read_file(from)
            .await
            .with_context(|| format!("reading image {}", from.display()))?;
        let format = ImageFormat::from_path(from)
            .with_context(|| format!("selecting image format for {}", from.display()))?;
        let img = ImageReader::with_format(Cursor::new(buf), format)
            .decode()
            .with_context(|| format!("reading image {}", from.display()))?;
        let img = img.resize(height, width, FilterType::Lanczos3);
        let mut buf = vec![];
        img.write_to(&mut Cursor::new(&mut buf), ImageFormat::WebP)
            .with_context(|| format!("converting image {}", from.display()))?;
        write_file(&to, buf)
            .await
            .with_context(|| format!("writing image {}", to.display()))?;
        Ok(())
    }

    async fn render_data(&mut self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[4/5]");
        pb.tick();
        pb.set_message(format!("{PAPER}rendering mod data"));
        let data = serde_json::to_string(&self.modules).context("generating JSON file")?;
        self.data = data.clone();
        write_file(&self.cfg.out_dir.join("mods.json"), data)
            .await
            .context("writing mods.json")?;
        pb.finish_with_message(format!(
            "{PAPER}rendered mods.json ({})",
            HumanBytes(self.data.len() as u64)
        ));
        Ok(())
    }

    async fn render_routes(&self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[5/5]");
        pb.tick();
        pb.set_message(format!("{LINK}rendering routes"));
        for r in &self.routes {
            pb.tick();
            pb.set_message(format!("{LINK}rendering {}", r.route.to_path()));
            if let Some(ref target) = r.redirect {
                write_file(
                    &self.cfg.out_dir.join(&r.path),
                    RedirectHtml { target }.render().with_context(|| {
                        format!("rendering redirect {} -> {}", r.route.to_path(), target)
                    })?,
                )
                .await
                .with_context(|| format!("writing file {}", r.path.display()))?;
            } else {
                let body = &self.render_route(r.route.clone()).await;
                write_file(
                    &self.cfg.out_dir.join(&r.path),
                    IndexHtml { head: "", body }.render().with_context(|| {
                        format!("rendering {} -> {}", r.route.to_path(), r.path.display())
                    })?,
                )
                .await
                .with_context(|| format!("writing file {}", r.path.display()))?;
            }
        }
        pb.finish_with_message(format!("{LINK}rendered {} routes", self.routes.len()));
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

    async fn create_directory(&self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[1/5]");
        if self.cfg.out_dir.exists() {
            pb.tick();
            pb.set_message(format!("{FOLDER}clearing output directory"));
            fs::remove_dir_all(&self.cfg.out_dir)
                .await
                .with_context(|| {
                    format!("clearing output directory {}", self.cfg.out_dir.display())
                })?;
        }
        pb.tick();
        pb.set_message(format!("{FOLDER}creating output directory"));
        fs::create_dir_all(&self.cfg.out_dir)
            .await
            .with_context(|| format!("creating output directory {}", self.cfg.out_dir.display()))?;
        pb.finish_with_message(format!("{FOLDER}created {}", self.cfg.out_dir.display()));
        Ok(())
    }

    async fn create_static_files(&self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[2/5]");
        for file in FILESYSTEM_STATIC {
            pb.tick();
            pb.set_message(format!("{PAPER}creating {}", file.path));
            file.create(&self.cfg.out_dir)
                .await
                .with_context(|| format!("creating {}", file.path))?;
        }
        pb.finish_with_message(format!(
            "{PAPER}created icons, styles, and scripts ({} files)",
            FILESYSTEM_STATIC.len()
        ));
        Ok(())
    }
}

fn from_image_error(e: ImageError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

fn web_path(p: &Path) -> String {
    p.components().fold(String::new(), |mut s, c| {
        match c {
            Component::Normal(cmp) => {
                let _ = write!(&mut s, "/{}", cmp.to_str().expect("invalid path"));
            }
            _ => panic!("bad path"),
        }
        s
    })
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
