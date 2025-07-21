use std::{env, fs::File, io, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::parse::Manifest;

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
}

impl Config {
    pub fn get(args: Args) -> io::Result<Self> {
        let args = match args.command {
            Some(Command::Generate(a)) => a,
            _ => args.generate,
        };
        let manifest_path = args
            .manifest
            .unwrap_or_else(|| env::current_dir().unwrap().join("faust/faust.yml"));
        let manifest = Manifest::from_yaml(File::open(&manifest_path)?)?;
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
        let fallback_dir = args.base_game_path.unwrap_or_else(|| src_dir.join(".."));

        Ok(Self {
            manifest,
            out_dir,
            src_dir,
            fallback_dir,
        })
    }
}
