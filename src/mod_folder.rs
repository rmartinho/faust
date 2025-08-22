use std::path::{Path, PathBuf};

use crate::{args::Config, parse::manifest::ParserMode::*};

#[derive(Clone)]
pub struct ModFolder {
    cfg: Config,
}

impl ModFolder {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub fn banner_png(&self) -> PathBuf {
        self.root_fallback(self.cfg.manifest.banner.to_str().unwrap())
    }

    pub fn descr_cultures_txt(&self) -> PathBuf {
        self.root_fallback("data/descr_cultures.txt")
    }
    pub fn descr_sm_factions_txt(&self) -> PathBuf {
        self.root_fallback("data/descr_sm_factions.txt")
    }
    pub fn descr_mount_txt(&self) -> PathBuf {
        self.root_fallback("data/descr_mount.txt")
    }
    pub fn descr_model_battle_txt(&self) -> PathBuf {
        self.root_fallback("data/descr_model_battle.txt")
    }
    pub fn export_descr_buildings_txt(&self) -> PathBuf {
        self.root_fallback("data/export_descr_buildings.txt")
    }
    pub fn export_descr_unit_txt(&self) -> PathBuf {
        self.root_fallback("data/export_descr_unit.txt")
    }
    pub fn text_expanded_txt(&self) -> PathBuf {
        self.root_fallback(match self.cfg.manifest.mode {
            Original | Remastered => "data/text/expanded_bi.txt",
            Medieval2 => "data/text/expanded.txt",
        })
    }
    pub fn text_export_units_txt(&self) -> PathBuf {
        self.root_fallback("data/text/export_units.txt")
    }
    pub fn ui_strategy_sd(&self) -> PathBuf {
        self.root_fallback("data/ui/strategy.sd")
    }
    pub fn ui_culture_spritesheet_tga(
        &self,
        culture: impl AsRef<Path>,
        path: impl AsRef<Path>,
    ) -> PathBuf {
        self.culture_fallback(culture, path)
    }

    pub fn descr_mercenaries_txt(&self) -> PathBuf {
        self.campaign_fallback("descr_mercenaries.txt")
    }
    pub fn descr_regions_txt(&self) -> PathBuf {
        self.campaign_fallback("descr_regions.txt")
    }
    pub fn descr_strat_txt(&self) -> PathBuf {
        self.campaign_fallback("descr_strat.txt")
    }
    pub fn radar_map_tga(&self) -> PathBuf {
        self.campaign_fallback(match self.cfg.manifest.mode {
            Original | Medieval2 => "radar_map2.tga",
            Remastered => "feral_radar_map.tga",
        })
    }
    pub fn map_regions_tga(&self) -> PathBuf {
        self.campaign_fallback("map_regions.tga")
    }
    pub fn unit_info_tga(&self, faction: &str, key: &str) -> PathBuf {
        let faction = faction.to_lowercase();
        let key = key.to_lowercase();
        self.generic_unit_fallback(if self.cfg.manifest.unit_info_images {
            let faction = if faction == "mercs" {
                "merc".into()
            } else {
                faction.to_lowercase()
            };
            existing_path(self.root_fallback(format!("data/ui/unit_info/{faction}/{key}_info.tga")))
                .unwrap_or(self.root_fallback(format!("data/ui/unit_info/merc/{key}_info.tga")))
        } else {
            let faction = faction.to_lowercase();
            existing_path(self.root_fallback(format!("data/ui/units/{faction}/#{key}.tga")))
                .unwrap_or(self.root_fallback(format!("data/ui/units/mercs/#{key}.tga")))
        })
    }
    pub fn faction_symbol_tga(&self, path: impl AsRef<Path>) -> PathBuf {
        self.maybe_missing_data_fallback(path)
    }

    fn root_fallback(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();
        existing_path(self.cfg.src_dir.join(path))
            .unwrap_or_else(|| self.cfg.fallback_dir.join(path))
    }
    fn maybe_missing_data_fallback(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();
        existing_path(self.cfg.src_dir.join(path))
            .or_else(|| existing_path(self.src_data_path().join(path)))
            .or_else(|| existing_path(self.cfg.fallback_dir.join(path)))
            .unwrap_or_else(|| self.fallback_data_path().join(path))
    }
    fn campaign_fallback(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();
        existing_path(self.src_campaign_path().join(path))
            .or_else(|| existing_path(self.src_maps_base_path().join(path)))
            .or_else(|| existing_path(self.fallback_campaign_path().join(path)))
            .unwrap_or_else(|| self.fallback_maps_base_path().join(path))
    }
    fn generic_unit_fallback(&self, path: impl AsRef<Path>) -> PathBuf {
        existing_path(path.as_ref())
            .map(Into::into)
            .unwrap_or_else(|| self.root_fallback("data/ui/generic/generic_unit_card.tga"))
    }
    fn culture_fallback(&self, culture: impl AsRef<Path>, path: impl AsRef<Path>) -> PathBuf {
        let culture = culture.as_ref();
        let path = path.as_ref();
        self.root_fallback(
            PathBuf::from("data/ui")
                .join(culture)
                .join("interface")
                .join(path),
        )
    }

    fn data_path(&self, root: impl AsRef<Path>) -> PathBuf {
        root.as_ref().join("data")
    }
    fn campaign_path(&self, root: impl AsRef<Path>) -> PathBuf {
        root.as_ref()
            .join("data/world/maps/campaign")
            .join(&self.cfg.manifest.campaign)
    }
    fn maps_base_path(&self, root: impl AsRef<Path>) -> PathBuf {
        root.as_ref().join("data/world/maps/base")
    }
    fn src_data_path(&self) -> PathBuf {
        self.data_path(&self.cfg.src_dir)
    }
    fn src_campaign_path(&self) -> PathBuf {
        self.campaign_path(&self.cfg.src_dir)
    }
    fn src_maps_base_path(&self) -> PathBuf {
        self.maps_base_path(&self.cfg.src_dir)
    }
    fn fallback_data_path(&self) -> PathBuf {
        self.data_path(&self.cfg.fallback_dir)
    }
    fn fallback_campaign_path(&self) -> PathBuf {
        self.campaign_path(&self.cfg.fallback_dir)
    }
    fn fallback_maps_base_path(&self) -> PathBuf {
        self.maps_base_path(&self.cfg.fallback_dir)
    }
}

fn existing_path<P: AsRef<Path>>(path: P) -> Option<P> {
    path.as_ref().exists().then_some(path)
}
