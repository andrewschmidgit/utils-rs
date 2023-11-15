use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg()]
    pub files: Option<Vec<PathBuf>>,

    #[arg(short, long)]
    pub all: bool,

    #[arg(short = 'A', long)]
    pub almost_all: bool,
}
