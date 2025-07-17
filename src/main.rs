#![feature(str_from_utf16_endian)]
#![allow(dead_code)]

use std::fs::File;

use clap::Parser;

use crate::{args::Args, parse::Manifest, render::Renderer};

mod args;
mod parse;
mod render;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let manifest_path = args
        .manifest
        .clone()
        .unwrap_or_else(|| "faust/faust.yml".into());
    let manifest = Manifest::from_yaml(File::open(&manifest_path)?)?;
    let modules = parse::parse_folder(&args, manifest).await?;

    let renderer = Renderer::new(&args, modules);
    renderer.render().await?;
    Ok(())
}
