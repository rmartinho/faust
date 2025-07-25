#![feature(str_from_utf16_endian)]
#![feature(path_add_extension)]
#![feature(panic_payload_as_str)]
#![feature(str_split_remainder)]
#![feature(str_split_whitespace_remainder)]
#![feature(pattern)]
#![allow(dead_code)]

use std::{
    fs::{File, metadata},
    time::{Duration, Instant},
};

use anyhow::Result;
use clap::Parser as _;
use console::style;
use indicatif::{HumanBytes, HumanDuration, ProgressBar};
use zip_dir::zip_dir;

use crate::{
    args::{Args, Command, Config},
    render::Renderer,
    serve::serve,
    utils::{CLAMP, LINK, LOOKING_GLASS, PACKAGE, SPARKLE, progress_style},
};

mod args;
mod pack;
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
    match args.command {
        Some(Command::Pack) => {
            return Ok(pack::pack().await?);
        }
        _ => {}
    }
    let started = Instant::now();
    let cfg = Config::get(args)?;

    let step = Instant::now();
    let modules = parse::parse_folder(&cfg).await?;
    println!(
        "{} {LOOKING_GLASS}{}",
        style("[1/3]").bold().dim(),
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
        style("[2/3]").bold().dim(),
        style(format!(
            "rendered site in {}",
            HumanDuration(step.elapsed())
        ))
        .green()
    );

    let step = Instant::now();
    let zip_file = cfg.out_dir.with_added_extension("zip");
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_message(format!("{CLAMP}zipping site"));
    pb.set_style(progress_style());
    zip_dir(&cfg.out_dir, File::create(&zip_file)?, None)?;
    pb.finish_and_clear();
    println!(
        "{} {CLAMP}{}",
        style("[3/3]").bold().dim(),
        style(format!("zipped site in {}", HumanDuration(step.elapsed()))).green()
    );

    println!(
        "      {SPARKLE}{}",
        style(format!("Done in {}", HumanDuration(started.elapsed()))).bold()
    );
    println!(
        "      {PACKAGE}Site files available at {} ({})",
        style(zip_file.display()).bold(),
        HumanBytes(metadata(&zip_file)?.len())
    );

    if cfg.serve {
        serve(&cfg).await?;
    }

    Ok(())
}
