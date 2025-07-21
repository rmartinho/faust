use std::{env, path::{Path, PathBuf}};

use copy_dir::copy_dir;
use tempdir::TempDir;
use tokio::fs;
use zip_dir::zip_dir;

pub async fn pack() -> Result<(), Box<dyn std::error::Error>> {
    let src_dir = env::current_dir()?;
    let tmp_dir = TempDir::new("faust")?;
    let dst_dir = tmp_dir.path().join("files");
    fs::create_dir_all(&dst_dir).await?;

    let _ = pack_dir(&src_dir, &dst_dir, "data/ui/units").await;
    let _ = pack_dir(&src_dir, &dst_dir, "data/ui/faction_icons").await;
    let _ = pack_dir(&src_dir, &dst_dir, "data/loading_screen/symbols").await;
    let _ = pack_dir(&src_dir, &dst_dir, "data/world/maps/base").await;
    let _ = pack_dir(&src_dir, &dst_dir, "data/world/maps/campaign").await;

    let _ = pack_file(&src_dir, &dst_dir, "data/text/expanded_bi.txt").await;
    let _ = pack_file(&src_dir, &dst_dir, "data/text/export_units.txt").await;
    let _ = pack_file(&src_dir, &dst_dir, "data/descr_sm_factions.txt").await;
    let _ = pack_file(&src_dir, &dst_dir, "data/export_descr_unit.txt").await;
    let _ = pack_file(&src_dir, &dst_dir, "data/export_descr_buildings.txt").await;

    env::set_current_dir(&dst_dir)?;

    println!("zipping");
    zip_dir(
        &PathBuf::from("."),
        std::fs::File::create(src_dir.join("faust-pack.zip"))?,
        None,
    )?;

    Ok(())
}

pub async fn pack_dir(src: &Path, dst: &Path, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("copying {dir}");
    let from = src.join(dir);
    let to = dst.join(dir);
    fs::create_dir_all(to.parent().unwrap()).await?;
    copy_dir(from, to)?;
    Ok(())
}

pub async fn pack_file(
    src: &Path,
    dst: &Path,
    file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("copying {file}");
    let from = src.join(file);
    let to = dst.join(file);
    fs::create_dir_all(to.parent().unwrap()).await?;
    fs::copy(from, to).await?;
    Ok(())
}
