use std::{
    collections::{BTreeSet, HashSet},
    fmt::{self, Display, Formatter, Write as _},
    path::{Component, Path, PathBuf},
};

use anyhow::{Context as _, Result};
use askama::Template as _;
use image::{
    DynamicImage, Rgba, RgbaImage,
    imageops::{FilterType::Lanczos3, filter3x3, overlay},
};
use implicit_clone::unsync::IString;
use indicatif::{HumanBytes, MultiProgress, ProgressBar};
use silphium::{
    ModuleMap, Route, StaticApp, StaticAppProps,
    model::{Aor, Era, Faction, Module, Pool, Region, Unit},
};
use tokio::fs;
use tracing::info;
use yew_router::Routable as _;

use crate::{
    args::Config,
    render::templates::{FILESYSTEM_STATIC, IndexHtml, PrefetchHtml, RedirectHtml},
    utils::{
        FOLDER, LINK, PAPER, PICTURE, path_fallback, progress_style, read_image, try_paths,
        write_file, write_image,
    },
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
            Self::render_image(&self.cfg, &src, &dst, MOD_BANNER_SIZE).await?;

            let radar_map_path = try_paths(
                &self.cfg,
                [
                    &format!(
                        "data/world/maps/campaign/{}/radar_map2.tga",
                        self.cfg.manifest.campaign
                    ),
                    "data/world/maps/base/radar_map2.tga",
                    "data/world/maps/campaign/imperial_campaign/radar_map2.tga",
                ],
            );
            let regions_map_path = try_paths(
                &self.cfg,
                [
                    &format!(
                        "data/world/maps/campaign/{}/map_regions.tga",
                        self.cfg.manifest.campaign
                    ),
                    "data/world/maps/base/map_regions.tga",
                ],
            );
            let mut areas = read_image(&self.cfg, regions_map_path).await?.into_rgba8();
            let radar = read_image(&self.cfg, radar_map_path).await?;
            let radar = radar
                .resize_exact(areas.width() * 2, areas.height() * 2, Lanczos3)
                .into_rgba8();
            erase_cities_and_ports(&mut areas);
            let mut rendered_mercs = HashSet::new();
            let mut pools = m.pools.to_vec();
            for p in pools.iter_mut() {
                let pool_path = Self::pool_path(&m.id, p);
                let dst = self.cfg.out_dir.join(&pool_path);
                pb.tick();
                pb.set_message(format!("{PICTURE}rendering {}", web_path(&pool_path)));
                Self::render_map(
                    &radar,
                    &areas,
                    &dst,
                    m.regions.values().filter(|r| p.regions.contains(&r.id)),
                    Rgba([0xFF, 0x71, 0x00, 0xC0]),
                    Rgba([0x00, 0x00, 0x00, 0xFF]),
                )
                .await?;

                let mut units = p.units.to_vec();
                for u in units.iter_mut() {
                    let src = path_fallback(
                        &self.cfg,
                        u.unit.image.as_ref(),
                        Some("data/ui/generic/generic_unit_card.tga"),
                    );
                    let portrait_path = Self::unit_portrait_path(&m.id, "mercs", &mut u.unit);
                    if !rendered_mercs.contains(&u.unit.id) {
                        rendered_mercs.insert(u.unit.id.clone());
                        let dst = self.cfg.out_dir.join(&portrait_path);
                        pb.tick();
                        pb.set_message(format!("{PICTURE}rendering {}", web_path(&portrait_path)));
                        Self::render_image(&self.cfg, &src, &dst, UNIT_PORTRAIT_SIZE).await?;
                    }
                }
                p.units = units.into();
            }
            m.pools = pools.into();

            for e in m.eras.values_mut() {
                let src = self.cfg.manifest_dir.join(e.icon.as_ref());
                let icon_path = Self::era_icon_path(&m.id, e);
                let dst = self.cfg.out_dir.join(&icon_path);
                pb.tick();
                pb.set_message(format!("{PICTURE}rendering {}", web_path(&icon_path)));
                Self::render_image(&self.cfg, &src, &dst, ERA_ICON_SIZE).await?;

                let src = self.cfg.manifest_dir.join(e.icoff.as_ref());
                let icoff_path = Self::era_icoff_path(&m.id, e);
                let dst = self.cfg.out_dir.join(&icoff_path);
                pb.tick();
                pb.set_message(format!("{PICTURE}rendering {}", web_path(&icoff_path)));
                Self::render_image(&self.cfg, &src, &dst, ERA_ICON_SIZE).await?;
            }

            let mut rendered_aors: HashSet<BTreeSet<IString>> = HashSet::new();
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
                Self::render_image(&self.cfg, &src, &dst, FACTION_SYMBOL_SIZE).await?;

                let mut aors = f.aors.to_vec();
                for aor in aors.iter_mut() {
                    let aor_path = Self::aor_path(&m.id, rendered_aors.len(), aor);
                    let region_set = aor.regions.iter().map(|s| s).collect();
                    if rendered_aors.contains(&region_set) {
                        continue;
                    }
                    let dst = self.cfg.out_dir.join(&aor_path);
                    pb.tick();
                    pb.set_message(format!("{PICTURE}rendering {}", web_path(&aor_path)));
                    Self::render_map(
                        &radar,
                        &areas,
                        &dst,
                        m.regions.values().filter(|r| aor.regions.contains(&r.id)),
                        Rgba([0xFF, 0x71, 0x00, 0xC0]),
                        Rgba([0x00, 0x00, 0x00, 0xFF]),
                    )
                    .await?;
                    rendered_aors.insert(region_set);
                }
                f.aors = aors.into();

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
                    Self::render_image(&self.cfg, &src, &dst, UNIT_PORTRAIT_SIZE).await?;
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

    fn era_icon_path(module_id: &str, era: &mut Era) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id)
            .join("eras")
            .join(era.icon.as_ref())
            .with_extension("webp");
        era.icon = web_path(&path).into();
        path
    }

    fn era_icoff_path(module_id: &str, era: &mut Era) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id)
            .join("eras")
            .join(era.icoff.as_ref())
            .with_extension("webp");
        era.icoff = web_path(&path).into();
        path
    }

    fn faction_symbol_path(module_id: &str, faction: &mut Faction) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id)
            .join("factions")
            .join(faction.id.as_ref())
            .with_extension("webp");
        faction.image = web_path(&path).into();
        path
    }

    fn unit_portrait_path(module_id: &str, faction_id: &str, unit: &mut Unit) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id)
            .join("units")
            .join(faction_id)
            .join(unit.key.as_ref())
            .with_extension("webp");
        unit.image = web_path(&path).into();
        path
    }

    fn pool_path(module_id: &str, pool: &mut Pool) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id)
            .join("pools")
            .join(pool.map.as_ref())
            .with_extension("webp");
        pool.map = web_path(&path).into();
        path
    }

    fn aor_path(module_id: &str, i: usize, aor: &mut Aor) -> PathBuf {
        let path = PathBuf::from("images")
            .join(module_id)
            .join(format!("aors/aor-{}.webp", i + 1));
        aor.map = web_path(&path).into();
        path
    }

    async fn render_image(
        cfg: &Config,
        from: &Path,
        to: &Path,
        (width, height): (u32, u32),
    ) -> Result<()> {
        let img = read_image(cfg, from).await?;
        let img = img.resize(width, height, Lanczos3);
        write_image(to, &img).await?;
        info!("rendered {}", to.display());
        Ok(())
    }

    async fn render_map<'a>(
        radar: &RgbaImage,
        areas: &RgbaImage,
        to: &Path,
        regions: impl IntoIterator<Item = &'a Region>,
        color: Rgba<u8>,
        border_color: Rgba<u8>,
    ) -> Result<()> {
        let mut image = DynamicImage::from(radar.clone());
        let regions: Vec<_> = regions.into_iter().collect();
        let regions = regions.into_iter();
        let mut blots = DynamicImage::from(RgbaImage::from_pixel(
            areas.width(),
            areas.height(),
            Rgba([0, 0, 0, 0]),
        ));
        for region in regions {
            let mut blot = RgbaImage::from_pixel(areas.width(), areas.height(), Rgba([0, 0, 0, 0]));
            let mut border =
                RgbaImage::from_pixel(areas.width(), areas.height(), Rgba([0, 0, 0, 0]));
            let region_color = Rgba([region.color.0, region.color.1, region.color.2, 0xFF]);
            areas
                .enumerate_pixels()
                .filter_map(|(x, y, p)| (region_color == *p).then_some((x, y)))
                .for_each(|(x, y)| {
                    blot.put_pixel(x, y, color);
                    border.put_pixel(x, y, border_color)
                });
            overlay(&mut blots, &blot, 0, 0);
            overlay(&mut blots, &filter3x3(&border, EDGE_KERNEL), 0, 0);
        }
        let blots = blots.resize(image.width(), image.height(), Lanczos3);
        overlay(&mut image, &blots, 0, 0);
        write_image(to, &DynamicImage::from(image)).await?;
        info!("rendered {}", to.display());
        Ok(())
    }

    async fn render_data(&mut self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[4/5]");
        pb.tick();
        pb.set_message(format!("{PAPER}rendering catalog data"));
        let mut data = vec![];
        ciborium::into_writer(&self.modules, &mut data).context("generating catalog file")?;
        self.data = data.clone();
        write_file(&self.cfg.out_dir.join("mods.cbor"), data)
            .await
            .context("writing mods.cbor")?;
        self.preload
            .push(("/mods.cbor".into(), PreloadType::Cbor.into()));
        pb.finish_with_message(format!(
            "{PAPER}rendered catalog ({})",
            HumanBytes(self.data.len() as u64)
        ));
        info!("rendered catalog");
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
        let path = route.to_path();
        let props = StaticAppProps {
            route,
            data: self.data.clone().into(),
        };
        let renderer = yew::LocalServerRenderer::<StaticApp>::with_props(props);
        let string = renderer.render().await;
        info!("rendered route {path}");
        string
    }

    fn render_preload(&self, r: &RenderRoute) -> Result<String> {
        let mut preload = self.preload.clone();
        preload.extend_from_slice(&r.preload);
        let string = PrefetchHtml { preload: &preload }.render()?;
        info!("rendered preloads");
        Ok(string)
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
        info!("created output directory");
        Ok(())
    }

    async fn create_static_files(&mut self, m: MultiProgress) -> Result<()> {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(progress_style());
        pb.set_prefix("[2/5]");
        pb.tick();
        pb.set_message(format!("{PAPER}writing faust.yml"));
        write_file(&self.cfg.out_dir.join("faust.yml"), &self.cfg.manifest.raw)
            .await
            .context("writing faust.yml")?;
        for file in FILESYSTEM_STATIC {
            pb.tick();
            pb.set_message(format!("{PAPER}creating {}", file.path));
            file.create(&self.cfg.out_dir)
                .await
                .with_context(|| format!("creating {}", file.path))?;
            info!("created static {}", file.path);
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

        routes.push(prepare_route(
            Route::Mercenaries {
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

fn erase_cities_and_ports(image: &mut RgbaImage) {
    let copy = image.clone();
    for (x, y, p) in image.enumerate_pixels_mut() {
        if *p == CITY_COLOR || *p == PORT_COLOR {
            for n in get_neighbors(&copy, x, y) {
                *p = n;
                break;
            }
        }
    }
}

fn get_neighbors(image: &RgbaImage, x: u32, y: u32) -> impl Iterator<Item = Rgba<u8>> {
    let x = x as i64;
    let y = y as i64;
    let coordinates = [
        (x, y - 1),
        (x, y + 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ];
    coordinates.into_iter().filter_map(|(x, y)| {
        if x < 0 || y < 0 || x as u32 >= image.width() || y as u32 >= image.height() {
            return None;
        }
        let p = image.get_pixel(x as u32, y as u32);
        (*p != WATER_COLOR).then_some(*p)
    })
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

const EDGE_KERNEL: &[f32] = &[-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];

const WATER_COLOR: Rgba<u8> = Rgba([0x29, 0x8C, 0xE9, 0xFF]);
const CITY_COLOR: Rgba<u8> = Rgba([0x00, 0x00, 0x00, 0xFF]);
const PORT_COLOR: Rgba<u8> = Rgba([0xFF, 0xFF, 0xFF, 0xFF]);
