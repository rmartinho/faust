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
    parse::{
        descr_mercenaries::Pool,
        descr_model_battle::Model,
        descr_mount::Mount,
        descr_regions::Region,
        export_descr_buildings::{Building, Requires},
        manifest::ParserMode,
        model::{RawModel, build_model},
    },
    utils::{LOOKING_GLASS, THINKING, path_fallback, progress_style, read_file, try_paths},
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

mod eval;
mod manifest;
mod model;
mod text;

pub async fn parse_folder(cfg: &Config) -> Result<ModuleMap> {
    let expanded_path = path_fallback(cfg, "data/text/expanded.txt", None);
    let expanded_bi_path = path_fallback(cfg, "data/text/expanded_bi.txt", None);
    let export_units_path = path_fallback(cfg, "data/text/export_units.txt", None);
    let descr_mercenaries_path = try_paths(
        cfg,
        [
            &format!(
                "data/world/maps/campaign/{}/descr_mercenaries.txt",
                cfg.manifest.campaign
            ),
            "data/world/maps/base/descr_mercenaries.txt",
        ],
    );
    let descr_regions_path = try_paths(
        cfg,
        [
            &format!(
                "data/world/maps/campaign/{}/descr_regions.txt",
                cfg.manifest.campaign
            ),
            "data/world/maps/base/descr_regions.txt",
        ],
    );
    let descr_sm_factions_path = path_fallback(cfg, "data/descr_sm_factions.txt", None);
    let export_descr_unit_path = path_fallback(cfg, "data/export_descr_unit.txt", None);
    let export_descr_buildings_path = path_fallback(cfg, "data/export_descr_buildings.txt", None);
    let descr_strat_path = try_paths(
        cfg,
        [
            &format!(
                "data/world/maps/campaign/{}/descr_strat.txt",
                cfg.manifest.campaign
            ),
            "data/world/maps/base/descr_strat.txt",
        ],
    );
    let descr_mount_path = path_fallback(cfg, "data/descr_mount.txt", None);
    let descr_model_battle_path = path_fallback(cfg, "data/descr_model_battle.txt", None);

    let m = MultiProgress::new();

    let expanded_text_path = match cfg.manifest.mode {
        ParserMode::Original | ParserMode::Remastered => expanded_bi_path,
        ParserMode::Medieval2 => expanded_path,
    };
    let mut text = parse_progress(
        m.clone(),
        1,
        expanded_text_path.clone(),
        parse_text(expanded_text_path, cfg.manifest.mode),
    )
    .await?;
    let export_units = parse_progress(
        m.clone(),
        2,
        export_units_path.clone(),
        parse_text(export_units_path, cfg.manifest.mode),
    )
    .await?;
    text.extend(export_units.into_iter());

    let pools = parse_progress(
        m.clone(),
        3,
        descr_mercenaries_path.clone(),
        parse_descr_mercenaries(descr_mercenaries_path, cfg.manifest.mode),
    )
    .await?;
    let regions = parse_progress(
        m.clone(),
        4,
        descr_regions_path.clone(),
        parse_descr_regions(descr_regions_path, cfg.manifest.mode),
    )
    .await?;
    let factions = parse_progress(
        m.clone(),
        5,
        descr_sm_factions_path.clone(),
        parse_descr_sm_factions(descr_sm_factions_path, cfg.manifest.mode),
    )
    .await?;
    let units = parse_progress(
        m.clone(),
        6,
        export_descr_unit_path.clone(),
        parse_export_descr_unit(export_descr_unit_path, cfg.manifest.mode),
    )
    .await?;
    let (require_aliases, buildings) = parse_progress(
        m.clone(),
        7,
        export_descr_buildings_path.clone(),
        parse_export_descr_buildings(export_descr_buildings_path, cfg.manifest.mode),
    )
    .await?;
    let strat = parse_progress(
        m.clone(),
        6,
        descr_strat_path.clone(),
        parse_descr_strat(descr_strat_path, cfg.manifest.mode),
    )
    .await?;
    let mounts = parse_progress(
        m.clone(),
        3,
        descr_mount_path.clone(),
        parse_descr_mount(descr_mount_path, cfg.manifest.mode),
    )
    .await?;
    let models = parse_progress(
        m.clone(),
        3,
        descr_model_battle_path.clone(),
        parse_descr_model_battle(descr_model_battle_path, cfg.manifest.mode),
    )
    .await?;

    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(progress_style());
    pb.set_prefix("[11/11]");
    pb.set_message(format!("{THINKING}building catalog..."));
    pb.enable_steady_tick(Duration::from_millis(200));
    let aliases = cfg
        .manifest
        .aliases
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let (factions, regions, pools, aors) = build_model(
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
            aors,
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
    i: usize,
    path: PathBuf,
    fut: impl Future<Output = T> + 'a,
) -> impl Future<Output = T> + 'a {
    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(progress_style());
    pb.set_prefix(format!("[{}/11]", i));
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

async fn parse_text(path: PathBuf, mode: ParserMode) -> Result<HashMap<String, String>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf16le_lossy(&buf).replace(BOM, "");
    text::parse(data, mode)
}

async fn parse_descr_mercenaries(path: PathBuf, mode: ParserMode) -> Result<Vec<Pool>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_mercenaries::parse(data, mode)
}

async fn parse_descr_regions(path: PathBuf, mode: ParserMode) -> Result<Vec<Region>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_regions::parse(data, mode)
}

async fn parse_descr_sm_factions(
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<descr_sm_factions::Faction>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_sm_factions::parse(data, mode)
}

async fn parse_export_descr_unit(
    path: PathBuf,
    mode: ParserMode,
) -> Result<Vec<export_descr_unit::Unit>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    export_descr_unit::parse(data, mode)
}

async fn parse_export_descr_buildings(
    path: PathBuf,
    mode: ParserMode,
) -> Result<(HashMap<String, Requires>, Vec<Building>)> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    export_descr_buildings::parse(data, mode)
}

async fn parse_descr_strat(path: PathBuf, mode: ParserMode) -> Result<HashMap<String, usize>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_strat::parse(data, mode)
}

async fn parse_descr_mount(path: PathBuf, mode: ParserMode) -> Result<HashMap<String, Mount>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_mount::parse(data, mode)
}

async fn parse_descr_model_battle(
    path: PathBuf,
    mode: ParserMode,
) -> Result<HashMap<String, Model>> {
    let buf = read_file(&path).await?;
    let data = String::from_utf8_lossy(&buf);
    descr_model_battle::parse(data, mode)
}

const BOM: &str = "\u{feff}";
