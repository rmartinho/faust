use cargo_emit::rerun_if_changed;
use std::process::Command;

fn main() {
    run_trunk();
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .use_cargo_dir_conventions()
        .process().unwrap();
}

fn run_trunk() {
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
