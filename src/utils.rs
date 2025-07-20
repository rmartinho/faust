use std::{io, path::Path};

use console::Emoji;
use indicatif::ProgressStyle;

pub const LOOKING_GLASS: Emoji = Emoji("🔍 ", "");
pub const PAPER: Emoji = Emoji("📃 ", "");
pub const LINK: Emoji = Emoji("🔗 ", "");
pub const SPARKLE: Emoji = Emoji("✨ ", ":-)");
pub const FOLDER: Emoji = Emoji("📁 ", "");
pub const PICTURE: Emoji = Emoji("🖼️  ", "");
pub const EARTH: Emoji = Emoji("🌍 ", "");
pub const CLAMP: Emoji = Emoji("🗜️  ", "");

pub async fn write_file(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let dir = path.as_ref().parent();
    if let Some(dir) = dir {
        tokio::fs::create_dir_all(dir).await?;
    }
    tokio::fs::write(path, contents).await
}

pub async fn read_file(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    tokio::fs::read(path).await
}

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
}
