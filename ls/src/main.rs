use std::io::{BufWriter, stdout};

use clap::Parser;
use ls::run;
use ls::cli::Cli;

fn main() {
    let args = Cli::parse();
    let mut writer = BufWriter::new(stdout());

    if let Err(e) = run(args, &mut writer) {
        eprintln!("{e}");
    }
}
