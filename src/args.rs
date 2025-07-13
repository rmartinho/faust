use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "faust/faust.yml")]
    pub manifest: Option<PathBuf>,
    #[arg(short, long, default_value = "faust/dist")]
    pub out_dir: Option<PathBuf>,
}
