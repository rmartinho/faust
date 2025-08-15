use std::{env, io::Cursor, path::PathBuf};

use crate::{parse::Manifest, platform};
use anyhow::{Context as _, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false, help = "verbose output")]
    pub verbose: bool,

    #[command(flatten)]
    pub generate: GenerateArgs,
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
    #[arg(long, help = "file to write list of used mod files")]
    pub deps_file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub manifest: Manifest,
    pub src_dir: PathBuf,
    pub out_dir: PathBuf,
    pub fallback_dir: PathBuf,
    pub manifest_dir: PathBuf,
    pub serve: bool,
    pub deps_file: Option<PathBuf>,
}

impl Config {
    pub fn get(args: Args) -> Result<Self> {
        let args = gen_args(args);
        let manifest_path = args.manifest.unwrap_or_else(|| {
            env::current_dir()
                .expect("current directory failed")
                .join("faust/faust.yml")
        });
        let manifest_text = std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("opening manifest at {}", manifest_path.display()))?;
        let manifest = Manifest::from_yaml(Cursor::new(&manifest_text))?;
        let manifest = Manifest {
            raw: manifest_text,
            ..manifest
        };
        let manifest_dir = manifest_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| env::current_dir().expect("current directory failed"));
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

        if let Some(deps_file) = &args.deps_file {
            std::fs::write(deps_file, format!("{}\n", manifest_path.display()))
                .context("creating deps file")?;
        }

        Ok(Self {
            manifest,
            out_dir,
            src_dir,
            fallback_dir,
            manifest_dir,
            serve: args.serve,
            deps_file: args.deps_file,
        })
    }
}

pub fn gen_args(args: Args) -> GenerateArgs {
    platform::prepare_generation_arguments(args)
}
