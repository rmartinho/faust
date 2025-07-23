use std::{env, fs, path::PathBuf};

use cargo_emit::rerun_if_changed;
use winresource::WindowsResource;

#[tokio::main]
async fn main() {
    build_win_resources();
    build_parsers();
    build_site_template().await
}

fn build_win_resources() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = WindowsResource::new();
        res.set_icon("faust.ico");
        res.compile().unwrap();
    }
}

fn build_parsers() {
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .use_cargo_dir_conventions()
        .process()
        .unwrap();
}

async fn build_site_template() {
    rerun_if_changed!("./silphium");
    let old_target = env::var("CARGO_TARGET_DIR").ok();
    let old_cd = env::current_dir().unwrap();
    unsafe { env::set_var("CARGO_TARGET_DIR", "target") };
    env::set_current_dir("silphium").unwrap();
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();

    let cfg = trunk::Trunk {
        action: trunk::TrunkSubcommands::Build(trunk::cmd::build::Build {
            release: Some(true),
            features: Some(vec!["hydration".into()]),
            dist: Some(out_dir.join("silphium_template")),
            filehash: Some(false),
            ..Default::default()
        }),
        verbose: 4,
        ..Default::default()
    };
    trunk::go(cfg).await.unwrap();

    if let Some(old_target) = old_target {
        unsafe { env::set_var("CARGO_TARGET_DIR", old_target) };
    }
    env::set_current_dir(old_cd).unwrap();
    fs::copy(out_dir.join("silphium_template/index.html"), "templates/index.html").unwrap();
}
