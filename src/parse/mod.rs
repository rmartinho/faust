use std::{collections::HashMap, path::PathBuf, time::Duration};

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar};
use silphium::{
    ModuleMap,
    model::{Era, Module},
};
use tracing::info;

use crate::{
    args::Config,
    mod_folder::ModFolder,
    parse::{
        descr_mercenaries::Pool,
        descr_model_battle::Model,
        descr_mount::Mount,
        descr_regions::Region,
        export_descr_buildings::{Building, Requires},
        manifest::ParserMode,
        model::{RawModel, build_model},
        sd::Sprite,
    },
    utils::{LOOKING_GLASS, THINKING, progress_style, read_file},
};
pub use manifest::Manifest;

mod descr_mercenaries;
mod descr_model_battle;
mod descr_mount;
mod descr_regions;
mod descr_sm_factions;
mod descr_strat;
mod export_descr_buildings;
mod export_descr_unit;
mod sd;
mod text;

mod eval;
mod model;

pub mod manifest;

pub async fn parse_folder(cfg: &Config) -> Result<ModuleMap> {
    let m = MultiProgress::new();

    let folder = ModFolder::new(cfg.clone());

    let text_expanded_txt = folder.text_expanded_txt();
    let mut text = parse_progress(
        m.clone(),
        text_expanded_txt.clone(),
        parse_text(cfg, text_expanded_txt, cfg.manifest.mode),
    )
    .await?;
    let text_export_units_txt = folder.text_export_units_txt();
    let export_units = parse_progress(
        m.clone(),
        text_export_units_txt.clone(),
        parse_text(cfg, text_export_units_txt, cfg.manifest.mode),
    )
    .await?;
    text.extend(export_units.into_iter());

    let descr_mercenaries_txt = folder.descr_mercenaries_txt();
    let pools = parse_progress(
        m.clone(),
        descr_mercenaries_txt.clone(),
        parse_descr_mercenaries(cfg, descr_mercenaries_txt, cfg.manifest.mode),
    )
    .await?;
    let descr_regions_txt = folder.descr_regions_txt();
    let regions = parse_progress(
        m.clone(),
        descr_regions_txt.clone(),
        parse_descr_regions(cfg, descr_regions_txt, cfg.manifest.mode),
    )
    .await?;
    let descr_sm_factions_txt = folder.descr_sm_factions_txt();
    let factions = parse_progress(
        m.clone(),
        descr_sm_factions_txt.clone(),
        parse_descr_sm_factions(cfg, descr_sm_factions_txt, cfg.manifest.mode),
    )
    .await?;
    let export_descr_unit_txt = folder.export_descr_unit_txt();
    let units = parse_progress(
        m.clone(),
        export_descr_unit_txt.clone(),
        parse_export_descr_unit(cfg, export_descr_unit_txt, cfg.manifest.mode),
    )
    .await?;
    let export_descr_buildings_txt = folder.export_descr_buildings_txt();
    let (require_aliases, buildings) = parse_progress(
        m.clone(),
        export_descr_buildings_txt.clone(),
        parse_export_descr_buildings(cfg, export_descr_buildings_txt, cfg.manifest.mode),
    )
    .await?;
    let descr_strat_txt = folder.descr_strat_txt();
    let strat = parse_progress(
        m.clone(),
        descr_strat_txt.clone(),
        parse_descr_strat(cfg, descr_strat_txt, cfg.manifest.mode),
    )
    .await?;
    let descr_mount_txt = folder.descr_mount_txt();
    let mounts = parse_progress(
        m.clone(),
        descr_mount_txt.clone(),
        parse_descr_mount(cfg, descr_mount_txt, cfg.manifest.mode),
    )
    .await?;
    let descr_model_battle_txt = folder.descr_model_battle_txt();
    let models = parse_progress(
        m.clone(),
        descr_model_battle_txt.clone(),
        parse_descr_model_battle(cfg, descr_model_battle_txt, cfg.manifest.mode),
    )
    .await?;
    let strategy_sd = folder.ui_strategy_sd();
    let sprites = parse_progress(
        m.clone(),
        strategy_sd.clone(),
        parse_sd(cfg, strategy_sd, cfg.manifest.mode),
    )
    .await?;

    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(progress_style());
    pb.set_message(format!("{THINKING}building catalog..."));
    pb.enable_steady_tick(Duration::from_millis(200));
    let aliases = cfg
        .manifest
        .aliases
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let (factions, regions, pools) = build_model(
        &cfg,
        RawModel {
            units,
            factions,
            regions,
            pools,
            buildings,
            require_aliases,
            text,
            strat,
            mounts,
            models,
        },
    );
    info!("built catalog");
    let _ = m.clear();

    let module_map = ModuleMap::from([(
        cfg.manifest.id.clone(),
        Module {
            id: cfg.manifest.id.clone(),
            name: cfg.manifest.name.clone(),
            banner: cfg.manifest.banner.to_string_lossy().into_owned().into(),
            factions,
            regions,
            pools,
            aliases,
            eras: cfg
                .manifest
                .eras
                .iter()
                .map(|(id, v)| {
                    (
                        id.clone(),
                        Era {
                            id: id.clone(),
                            icon: v
                                .icon
                                .clone()
                                .unwrap_or_else(|| {
                                    PathBuf::from("eras")
                                        .join(id.as_ref())
                                        .with_added_extension("png")
                                })
                                .to_string_lossy()
                                .into_owned()
                                .into(),
                            icoff: v
                                .icoff
                                .clone()
                                .unwrap_or_else(|| {
                                    PathBuf::from("eras").join(format!("{id}-off.png"))
                                })
                                .to_string_lossy()
                                .into_owned()
                                .into(),
                            name: v.name.clone().unwrap_or(id.clone()),
                        },
                    )
                })
                .collect(),
        },
    )]);
    Ok(module_map)
}

fn parse_progress<'a, T>(
    m: MultiProgress,
    path: PathBuf,
    fut: impl Future<Output = T> + 'a,
) -> impl Future<Output = T> + 'a {
    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(progress_style());
    pb.set_message(format!("{LOOKING_GLASS}parsing {}...", path.display()));

    async move {
        let res = fut.await;
        pb.finish_with_message(format!(
            "{LOOKING_GLASS}parsing {}... done.",
            path.display()
        ));
        info!("parsed {}", path.display());
        res
    }
}

async fn parse_text(
    cfg: &Config,
    mut path: PathBuf,
    mode: ParserMode,
) -> Result<HashMap<String, String>> {
    if mode == ParserMode::Medieval2 && !path.exists() {
        path.add_extension("strings.bin");
        let buf = read_file(cfg, &path).await?;
        text::parse_bin(buf, mode)
    } else {
        let buf = read_file(cfg, &path).await?;
        let data = String::from_utf16le_lossy(&buf).replace(BOM, "");
        text::parse_txt(data, mode)
    }
}

async fn parse_descr_mercenaries(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<Pool>> {
    let buf = read_file(cfg, &path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_mercenaries::parse(data, mode)
}

async fn parse_descr_regions(cfg: &Config, path: PathBuf, mode: ParserMode) -> Result<Vec<Region>> {
    let buf = read_file(cfg, &path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_regions::parse(data, mode)
}

async fn parse_descr_sm_factions(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<descr_sm_factions::Faction>> {
    let buf = read_file(cfg, &path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_sm_factions::parse(data, mode)
}

async fn parse_export_descr_unit(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<export_descr_unit::Unit>> {
    let buf = read_file(cfg, &path).await?;
    let data = String::from_utf8_lossy(&buf);
    export_descr_unit::parse(data, mode)
}

async fn parse_export_descr_buildings(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<(HashMap<String, Requires>, Vec<Building>)> {
    let buf = read_file(cfg, &path).await?;
    let data = String::from_utf8_lossy(&buf);
    export_descr_buildings::parse(data, mode)
}

async fn parse_descr_strat(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<HashMap<String, usize>> {
    let buf = read_file(cfg, &path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_strat::parse(data, mode)
}

async fn parse_descr_mount(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<HashMap<String, Mount>> {
    if cfg.manifest.estimate_speed() {
        let buf = read_file(cfg, &path).await?;
        let data = String::from_utf8_lossy(&buf);
        descr_mount::parse(data, mode)
    } else {
        Ok(HashMap::new())
    }
}

async fn parse_descr_model_battle(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<HashMap<String, Model>> {
    if cfg.manifest.estimate_speed() {
        let buf = read_file(cfg, &path).await?;
        let data = String::from_utf8_lossy(&buf);
        descr_model_battle::parse(data, mode)
    } else {
        Ok(HashMap::new())
    }
}

async fn parse_sd(
    cfg: &Config,
    path: PathBuf,
    mode: ParserMode,
) -> Result<HashMap<String, Sprite>> {
    if cfg.manifest.mode == ParserMode::Medieval2 {
        let buf = read_file(cfg, &path).await?;
        sd::parse(buf, mode)
    } else {
        Ok(HashMap::new())
    }
}

const BOM: &str = "\u{feff}";
