use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use console::Emoji;
use indicatif::ProgressStyle;
use tokio::fs;

use crate::args::Config;

pub const LOOKING_GLASS: Emoji = Emoji("🔍 ", "");
pub const PAPER: Emoji = Emoji("📃 ", "");
pub const LINK: Emoji = Emoji("🔗 ", "");
pub const SPARKLE: Emoji = Emoji("✨ ", ":-) ");
pub const FOLDER: Emoji = Emoji("📁 ", "");
pub const PICTURE: Emoji = Emoji("🖼️  ", "");
pub const EARTH: Emoji = Emoji("🌍 ", "(#) ");
pub const CLAMP: Emoji = Emoji("🗜️  ", "");
pub const THINKING: Emoji = Emoji("💭  ", "");
pub const PACKAGE: Emoji = Emoji("📦 ", "[+] ");

pub async fn write_file(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    let dir = path.parent();
    if let Some(dir) = dir {
        fs::create_dir_all(dir)
            .await
            .with_context(|| format!("creating {}", dir.display()))?;
    }
    Ok(fs::write(path, contents)
        .await
        .with_context(|| format!("creating {}", path.display()))?)
}

pub async fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = path.as_ref();
    Ok(fs::read(path)
        .await
        .with_context(|| format!("reading {}", path.display()))?)
}

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .expect("invalid progress style")
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
}

pub fn path_fallback(cfg: &Config, path: &str, generic: Option<&str>) -> PathBuf {
    let first = cfg.src_dir.join(path);
    let fallback = cfg.fallback_dir.join(path);
    if first.exists() {
        first
    } else if fallback.exists() {
        fallback
    } else if let Some(generic) = generic {
        cfg.fallback_dir.join(generic)
    } else {
        first
    }
}
