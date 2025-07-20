use std::{env, fs::File, io, path::PathBuf};

use clap::Parser;

use crate::parse::Manifest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "faust/faust.yml")]
    pub manifest: Option<PathBuf>,
    #[arg(short, long, default_value = "site")]
    pub out_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub manifest: Manifest,
    pub src_dir: PathBuf,
    pub out_dir: PathBuf,
}

impl Config {
    pub fn get() -> io::Result<Self> {
        let args = Args::parse();
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
            .unwrap_or(manifest_dir);
        let out_dir = args
            .out_dir
            .unwrap_or_else(|| env::current_dir().unwrap().join("faust"));

        Ok(Self {
            manifest,
            out_dir,
            src_dir,
        })
    }
}
