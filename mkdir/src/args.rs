use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg()]
    pub directory: Vec<PathBuf>,

    #[arg(short, long)]
    pub parents: bool,

    #[arg(short, long)]
    pub mode: Option<String>,
}
