use std::{
    fmt::{self, Display, Formatter, Write as _},
    io::Cursor,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context as _, Result};
use askama::Template as _;
use image::{ImageFormat, ImageReader, imageops::FilterType};
use implicit_clone::unsync::IString;
use indicatif::{HumanBytes, MultiProgress, ProgressBar};
use silphium::{
    ModuleMap, Route, StaticApp, StaticAppProps,
    model::{Era, Faction, Module, Unit},
};
use tokio::fs;
use yew_router::Routable as _;

use crate::{
    args::Config,
    render::templates::{FILESYSTEM_STATIC, IndexHtml, PrefetchHtml, RedirectHtml},
    utils::{FOLDER, LINK, PAPER, PICTURE, path_fallback, progress_style, read_file, write_file},
};

#[derive(Clone)]
pub struct Preload {
    pub r#as: PreloadAs,
    pub r#type: PreloadType,
    pub cors: bool,
}

impl From<PreloadType> for Preload {
    fn from(value: PreloadType) -> Self {
        Self {
            r#as: match value {
                PreloadType::Wasm | PreloadType::Json | PreloadType::Cbor => PreloadAs::Fetch,
                PreloadType::Woff | PreloadType::Woff2 => PreloadAs::Font,
                PreloadType::Svg | PreloadType::Png | PreloadType::Webp => PreloadAs::Image,
            },
            r#type: value,
            cors: match value {
                PreloadType::Wasm
                | PreloadType::Json
                | PreloadType::Cbor
                | PreloadType::Woff
                | PreloadType::Woff2 => true,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum PreloadAs {
    Fetch,
    Font,
    Image,
}

impl Display for PreloadAs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch => write!(f, "fetch"),
            Self::Font => write!(f, "font"),
            Self::Image => write!(f, "image"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum PreloadType {
    Wasm,
    Cbor,
    Json,
    Woff,
    Woff2,
    Svg,
    Png,
    Webp,
}

impl Display for PreloadType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wasm => write!(f, "application/wasm"),
            Self::Cbor => write!(f, "application/cbor"),
            Self::Json => write!(f, "application/json"),
            Self::Woff => write!(f, "application/font-woff"),
            Self::Woff2 => write!(f, "font/woff2"),
            Self::Svg => write!(f, "image/svg+xml"),
            Self::Png => write!(f, "image/png"),
            Self::Webp => write!(f, "image/webp"),
        }
    }
}

#[derive(Clone)]
pub struct Renderer {
    pub cfg: Config,
    pub data: Vec<u8>,
    pub modules: ModuleMap,
    pub preload: Vec<(String, Preload)>,
}

impl Renderer {
    pub fn new(cfg: &Config, modules: ModuleMap) -> Self {
        Self {
            cfg: cfg.clone(),
            data: Vec::new(),
            modules,
            preload: vec![],
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

            for e in m.eras.values_mut() {
                let src = self.cfg.manifest_dir.join(e.icon.as_ref());
                let icon_path = Self::era_icon_path(&m.id, e);
                let dst = self.cfg.out_dir.join(&icon_path);
                pb.tick();
                pb.set_message(format!("{PICTURE}rendering {}", web_path(&banner_path)));
                Self::render_image(&src, &dst, ERA_ICON_SIZE).await?;

                let src = self.cfg.manifest_dir.join(e.icoff.as_ref());
                let icoff_path = Self::era_icoff_path(&m.id, e);
                let dst = self.cfg.out_dir.join(&icoff_path);
                pb.tick();
                pb.set_message(format!("{PICTURE}rendering {}", web_path(&banner_path)));
                Self::render_image(&src, &dst, ERA_ICON_SIZE).await?;
            }

            for f in m.factions.values_mut() {
                let src = path_fallback(
                    &self.cfg,
                    f.image.as_ref(),
                    Some("data/loading_screen/symbols/symbol128_slaves.tga"),
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
                        Some("data/ui/generic/generic_unit_card.tga"),
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

    fn era_icon_path(module_id: &IString, era: &mut Era) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id.as_ref())
            .join("eras")
            .join(era.icon.as_ref())
            .with_extension("webp");
        era.icon = web_path(&path).into();
        path
    }

    fn era_icoff_path(module_id: &IString, era: &mut Era) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id.as_ref())
            .join("eras")
            .join(era.icoff.as_ref())
            .with_extension("webp");
        era.icoff = web_path(&path).into();
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
        let mut data = vec![];
        ciborium::into_writer(&self.modules, &mut data).context("generating JSON file")?;
        self.data = data.clone();
        write_file(&self.cfg.out_dir.join("mods.cbor"), data)
            .await
            .context("writing mods.cbor")?;
        self.preload
            .push(("/mods.cbor".into(), PreloadType::Cbor.into()));
        pb.finish_with_message(format!(
            "{PAPER}rendered mods.cbor ({})",
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
        let routes = collect_routes(&self.modules);
        for r in &routes {
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
                let head = &self
                    .render_preload(r)
                    .with_context(|| format!("rendering preloads for {}", r.route.to_path()))?;
                write_file(
                    &self.cfg.out_dir.join(&r.path),
                    IndexHtml { head, body }.render().with_context(|| {
                        format!("rendering {} -> {}", r.route.to_path(), r.path.display())
                    })?,
                )
                .await
                .with_context(|| format!("writing file {}", r.path.display()))?;
            }
        }
        pb.finish_with_message(format!("{LINK}rendered {} routes", routes.len()));
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

    fn render_preload(&self, r: &RenderRoute) -> Result<String> {
        let mut preload = self.preload.clone();
        preload.extend_from_slice(&r.preload);
        Ok(PrefetchHtml { preload: &preload }.render()?)
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

    async fn create_static_files(&mut self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[2/5]");
        for file in FILESYSTEM_STATIC {
            pb.tick();
            pb.set_message(format!("{PAPER}creating {}", file.path));
            file.create(&self.cfg.out_dir)
                .await
                .with_context(|| format!("creating {}", file.path))?;
            if let Some(preload_as) = file.preload_as {
                self.preload.push((file.path.into(), preload_as.into()));
            }
        }
        pb.finish_with_message(format!(
            "{PAPER}created icons, styles, and scripts ({} files)",
            FILESYSTEM_STATIC.len()
        ));
        Ok(())
    }
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
    pub preload: Vec<(String, Preload)>,
}

fn collect_routes(modules: &ModuleMap) -> Vec<RenderRoute> {
    let mut routes = Vec::new();
    routes.push(RenderRoute {
        route: Route::Home,
        path: "index.html".into(),
        redirect: None,
        preload: vec![],
    });
    routes.push(RenderRoute {
        route: Route::NotFound,
        path: "404.html".into(),
        redirect: None,
        preload: vec![],
    });

    for module in modules.values() {
        routes.push(prepare_route(
            Route::Module {
                module: module.id.clone(),
            },
            vec![],
        ));

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
            }
            routes.push(prepare_route(route, vec![]));
        }
    }
    routes
}

fn prepare_route(route: Route, preload: Vec<(String, Preload)>) -> RenderRoute {
    let path = route.to_path();
    let path = PathBuf::from(&path[1..]).join("index.html");
    RenderRoute {
        path,
        route,
        redirect: None,
        preload,
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
        preload: vec![],
    }
}

const MOD_BANNER_SIZE: (u32, u32) = (512, 256);
const ERA_ICON_SIZE: (u32, u32) = (64, 64);
const FACTION_SYMBOL_SIZE: (u32, u32) = (128, 128);
const UNIT_PORTRAIT_SIZE: (u32, u32) = (82, 112);
