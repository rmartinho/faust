use std::fs::File;

use clap::Parser;

use crate::{args::Args, env::Env, parse::manifest::Manifest};

mod args;
mod env;
mod parse;
mod render;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::new(Args::parse());

    let _manifest = Manifest::from_yaml(File::open(&env.manifest_path)?)?;

    render::create_directory(&env).await?;
    render::create_static_files(&env).await?;
    render::render_routes(&env).await?;
    Ok(())
}
