#![feature(str_from_utf16_endian)]
#![feature(path_add_extension)]
#![feature(panic_payload_as_str)]
#![feature(str_split_remainder)]
#![feature(str_split_whitespace_remainder)]
#![feature(pattern)]
#![feature(try_blocks)]
#![allow(dead_code)]

use std::time::Instant;

use anyhow::Result;
use clap::Parser as _;
use console::style;
use indicatif::HumanDuration;

use crate::{
    args::{Args, Config},
    render::Renderer,
    serve::serve,
    utils::{LINK, LOOKING_GLASS, PACKAGE, SPARKLE},
};

mod args;
mod parse;
mod platform;
mod render;
mod serve;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    platform::set_panic_hook();

    Ok(platform::finish(run().await)?)
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let started = Instant::now();
    let cfg = Config::get(args)?;

    let step = Instant::now();
    let modules = parse::parse_folder(&cfg).await?;
    println!(
        "{} {LOOKING_GLASS}{}",
        style("[1/2]").bold().dim(),
        style(format!(
            "parsed mod folder in {}",
            HumanDuration(step.elapsed())
        ))
        .green()
    );

    let step = Instant::now();
    let mut renderer = Renderer::new(&cfg, modules);
    renderer.render().await?;
    println!(
        "{} {LINK}{}",
        style("[2/2]").bold().dim(),
        style(format!(
            "rendered site in {}",
            HumanDuration(step.elapsed())
        ))
        .green()
    );

    println!(
        "      {SPARKLE}{}",
        style(format!("Done in {}", HumanDuration(started.elapsed()))).bold()
    );
    println!(
        "      {PACKAGE}Site files available at {}",
        style(cfg.out_dir.display()).bold(),
    );

    if cfg.serve {
        serve(&cfg).await?;
    }

    Ok(())
}
