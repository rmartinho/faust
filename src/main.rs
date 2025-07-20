#![feature(str_from_utf16_endian)]
#![allow(dead_code)]

use crate::{args::Config, render::Renderer};

mod args;
mod parse;
mod render;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::get()?;
    let modules = parse::parse_folder(&cfg).await?;

    let mut renderer = Renderer::new(&cfg, modules);
    renderer.render().await?;

    Ok(())
}
