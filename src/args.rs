use std::{env, fs::File, io, path::PathBuf};

use crate::parse::Manifest;
use clap::{Parser, Subcommand};

#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::MAX_PATH,
        UI::Controls::Dialogs::{GetOpenFileNameW, OPENFILENAMEW},
    },
    core::w,
};
#[cfg(windows)]
use windows_strings::PWSTR;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    generate: GenerateArgs,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Args, Debug)]
pub struct GenerateArgs {
    #[arg(help = "the manifest file")]
    pub manifest: Option<PathBuf>,
    #[arg(short, long, help = "where to output the site")]
    pub out_dir: Option<PathBuf>,
    #[arg(short, long, help = "base game path (for fallbacks)")]
    pub base_game_path: Option<PathBuf>,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "serve the site after generation"
    )]
    pub serve: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Generates a FAUST website")]
    Generate(GenerateArgs),
    #[command(about = "Packs all the mod files used (generally useful for bug reports)")]
    Pack,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub manifest: Manifest,
    pub src_dir: PathBuf,
    pub out_dir: PathBuf,
    pub fallback_dir: PathBuf,
    pub serve: bool,
}

impl Config {
    pub fn get(args: Args) -> io::Result<Self> {
        let args = gen_args(args);
        let manifest_path = args
            .manifest
            .unwrap_or_else(|| env::current_dir().unwrap().join("faust/faust.yml"));
        let manifest = Manifest::from_yaml(File::open(&manifest_path).unwrap())?;
        let manifest_dir = manifest_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| env::current_dir().unwrap());
        let src_dir = manifest
            .dir
            .clone()
            .map(|d| manifest_dir.join(d))
            .or_else(|| manifest_dir.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| manifest_dir.clone());
        let out_dir = args.out_dir.unwrap_or_else(|| manifest_dir.join("site"));
        let fallback_dir = args
            .base_game_path
            .or_else(|| src_dir.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| "..".into());

        Ok(Self {
            manifest,
            out_dir,
            src_dir,
            fallback_dir,
            serve: args.serve,
        })
    }
}

pub fn gen_args(args: Args) -> GenerateArgs {
    if cfg!(not(windows)) || has_args() {
        match args.command {
            Some(Command::Generate(a)) => a,
            _ => args.generate,
        }
    } else {
        #[cfg(not(windows))]
        {
            unreachable!()
        }
        #[cfg(windows)]
        {
            let mut file = vec![0; MAX_PATH as _];
            let mut ofn = OPENFILENAMEW {
                lStructSize: std::mem::size_of::<OPENFILENAMEW>() as _,
                lpstrFilter: w!("Manifest file\0faust.yml\0"),
                lpstrTitle: w!("Select a manifest file"),
                nMaxFile: file.len() as _,
                lpstrFile: PWSTR(file.as_mut_ptr()),
                ..Default::default()
            };
            let success: bool = unsafe { GetOpenFileNameW(&mut ofn) }.into();
            if !success {
                panic!("canceled open file dialog")
            }
            let n = file.iter().position(|c| *c == 0).unwrap();
            file.truncate(n);
            let path = String::from_utf16(&file).unwrap().into();
            GenerateArgs {
                manifest: Some(path),
                out_dir: None,
                base_game_path: None,
                serve: true,
            }
        }
    }
}

fn has_args() -> bool {
    env::args().len() > 1
}
