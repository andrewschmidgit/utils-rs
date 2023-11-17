use std::process::exit;

use clap::Parser;
use mkdir::{args, run};

fn main() {
    let args = args::Args::parse();

    let result = run(args);

    if let Err(e) = result {
        eprintln!("{}", e);
        exit(1);
    }
}
