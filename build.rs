use cargo_emit::rerun_if_changed;
use std::{os::unix::process::CommandExt, process::Command};

fn main() {
    rerun_if_changed!("./silphium");
    let _ = Command::new("trunk")
        .arg("build")
        .arg("--release")
        .args(["--features", "hydration"])
        .current_dir("./silphium")
        .env("TRUNK_BUILD_DIST", "../dist")
        .env("TRUNK_BUILD_FILEHASH", "false")
        .exec();
}
