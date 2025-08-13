use std::{
    convert::FloatToInt,
    io::Cursor,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context as _, Error, Result};
use console::Emoji;
use image::{DynamicImage, ImageFormat, ImageReader};
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

pub async fn read_image(path: impl AsRef<Path>) -> Result<DynamicImage> {
    let from = path.as_ref();
    let buf = read_file(from)
        .await
        .with_context(|| format!("reading image {}", from.display()))?;
    let format = ImageFormat::from_path(from)
        .with_context(|| format!("selecting image format for {}", from.display()))?;
    ImageReader::with_format(Cursor::new(buf), format)
        .decode()
        .with_context(|| format!("reading image {}", from.display()))
}

pub async fn write_image(path: impl AsRef<Path>, img: &DynamicImage) -> Result<()> {
    let to = path.as_ref();
    let mut buf = vec![];
    img.write_to(&mut Cursor::new(&mut buf), ImageFormat::WebP)
        .with_context(|| format!("converting to image {}", to.display()))?;
    write_file(&to, buf)
        .await
        .with_context(|| format!("writing image {}", to.display()))
}

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

fn do_try_paths<'a>(root: &Path, paths: &[&'a str]) -> PathBuf {
    for path in paths.as_ref().iter() {
        let file = root.join(path);
        if file.exists() {
            return file;
        }
    }
    return paths.as_ref()[0].into();
}

pub fn try_paths<'a>(cfg: &Config, paths: impl AsRef<[&'a str]>) -> PathBuf {
    let first = do_try_paths(&cfg.src_dir, paths.as_ref());
    if first.exists() {
        first
    } else {
        do_try_paths(&cfg.fallback_dir, paths.as_ref())
    }
}

pub fn path_fallback(cfg: &Config, path: &str, generic: Option<&str>) -> PathBuf {
    [
        cfg.src_dir.join(path),
        cfg.src_dir.join("data").join(path),
        cfg.fallback_dir.join(path),
        cfg.fallback_dir.join("data").join(path),
    ]
    .into_iter()
    .chain(generic.into_iter().map(|g| cfg.fallback_dir.join(g)))
    .find(|p| p.exists())
    .unwrap_or(cfg.src_dir.join(path))
}

pub fn parse_maybe_float_int<I>(s: &str) -> Result<I>
where
    f64: FloatToInt<I>,
    I: FromStr,
{
    Ok(s.parse().or_else(|_| {
        Ok::<_, Error>(
            s.parse::<f64>()
                .map(|f| unsafe { f.ceil().to_int_unchecked() })?,
        )
    })?)
}
