#![feature(str_from_utf16_endian)]
#![feature(path_add_extension)]
#![feature(str_split_remainder)]
#![feature(str_split_whitespace_remainder)]
#![feature(pattern)]
#![feature(try_blocks)]
#![feature(convert_float_to_int)]
#![allow(dead_code)]

use std::{io, time::Instant};

use anyhow::Result;
use clap::Parser as _;
use console::style;
use indicatif::HumanDuration;
use tracing_subscriber::{filter, fmt::time::ChronoLocal, prelude::*};

use crate::{
    args::{Args, Config},
    render::Renderer,
    serve::serve,
    utils::{LINK, LOOKING_GLASS, PACKAGE, SPARKLE},
};

mod args;
mod mod_folder;
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

fn setup_tracing(args: &Args) -> Result<()> {
    if args.verbose {
        let stderr_log_level = filter::LevelFilter::INFO;
        let stderr_layer = tracing_subscriber::fmt::layer()
            .with_writer(io::stderr);

        tracing_subscriber::registry()
            .with(
                stderr_layer
                    .with_timer(ChronoLocal::rfc_3339())
                    .with_filter(stderr_log_level),
            )
            .try_init()?;
    }
    Ok(())
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let started = Instant::now();

    setup_tracing(&args)?;

    let cfg = Config::get(args)?;

    let step = Instant::now();
    let modules = parse::parse_folder(&cfg).await?;
    println!(
        "{LOOKING_GLASS}{}",
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
        "{LINK}{}",
        style(format!(
            "rendered site in {}",
            HumanDuration(step.elapsed())
        ))
        .green()
    );

    println!(
        "{SPARKLE}{}",
        style(format!("Done in {}", HumanDuration(started.elapsed()))).bold()
    );
    println!(
        "{PACKAGE}Site files available at {}",
        style(cfg.out_dir.display()).bold(),
    );

    if cfg.serve {
        serve(&cfg).await?;
    }

    Ok(())
}
