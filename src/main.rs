#![feature(str_from_utf16_endian)]
#![feature(path_add_extension)]
#![feature(panic_payload_as_str)]
#![allow(dead_code)]

use std::{
    fs::{File, metadata},
    time::{Duration, Instant},
};

use clap::Parser as _;
use console::style;
use indicatif::{HumanBytes, HumanDuration, ProgressBar};
#[cfg(windows)]
use windows::{
    Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW},
    core::w,
};
#[cfg(windows)]
use windows_strings::HSTRING;
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
mod render;
mod serve;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        std::panic::set_hook(Box::new(move |info| {
            use std::fmt::Write as _;
            let mut message = String::from("An error has occurred.\n\n");
            if let Some(s) = info.payload_as_str() {
                let _ = write!(message, "{s:?}\n");
            } else {
                let _ = write!(message, "Panic occurred {info:?}.\n");
            };
            if let Some(loc) = info.location() {
                let _ = write!(message, "\t@ {loc}");
            }

            unsafe {
                MessageBoxW(
                    None,
                    &HSTRING::from(message),
                    w!("Error"),
                    MB_OK | MB_ICONERROR,
                )
            };
            dont_disappear::enter_to_continue::default();
        }));
    }

    let res = run().await;

    #[cfg(windows)]
    {
        if let Err(ref e) = res {
            eprintln!("{:?}", e);
        }
        dont_disappear::enter_to_continue::default();
    }

    res
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
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
    zip_dir(&cfg.out_dir, File::create(&zip_file)?, None).unwrap();
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
        HumanBytes(metadata(&zip_file).unwrap().len())
    );

    if cfg.serve {
        serve(&cfg).await;
    }

    Ok(())
}
