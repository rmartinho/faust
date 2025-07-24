use std::path::{Path, PathBuf};

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

pub async fn write_file(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> anyhow::Result<()> {
    let dir = path.as_ref().parent();
    if let Some(dir) = dir {
        fs::create_dir_all(dir).await?;
    }
    Ok(fs::write(path, contents).await?)
}

pub async fn read_file(path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path).await?)
}

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .expect("invalid progress style")
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
}
 
pub fn path_fallback(cfg: &Config, path: &str) -> PathBuf {
    let first = cfg.src_dir.join(path);
    if first.exists() {
        first
    } else {
        cfg.fallback_dir.join(path)
    }
}
