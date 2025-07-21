use cargo_emit::rerun_if_changed;
use std::{env, process::Command};
use winresource::WindowsResource;

fn main() {
    build_win_resources();
    build_site_template();
    build_parsers();
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

fn build_site_template() {
    rerun_if_changed!("./silphium");
    let _ = Command::new("trunk")
        .arg("build")
        .arg("--release")
        .args(["--features", "hydration"])
        .current_dir("./silphium")
        .env("TRUNK_BUILD_DIST", "../dist")
        .env("TRUNK_BUILD_FILEHASH", "false")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
