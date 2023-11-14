use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg()]
    pub files: Option<Vec<PathBuf>>,
}
