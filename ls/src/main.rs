use std::{io::{self, Stdout, Write}, error::Error, fs, path::PathBuf};

use clap::Parser;
use ls::cli::Cli;

fn main() {
    let args = Cli::parse();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    let write_result = list(args, &mut handle);

    if let Err(err) = write_result {
        println!("Error while printing: {}", err);
    }
}

fn list(args: Cli, handle: &mut io::BufWriter<Stdout>) -> Result<(), Box<dyn Error>> {
    let files = match args.files {
        Some(f) => f,
        None => vec![PathBuf::new()],
    };

    let total_file_count = files.len();

    let mut file_names = files.iter()
        .filter(|f| f.is_file())
        .map(|f| f.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    file_names.sort_unstable();
    let file_names = file_names;

    for f in file_names.iter() {
        write!(handle, "{f} ")?;
    }

    if !file_names.is_empty() {
        writeln!(handle)?;
    }

    let mut directories: Vec<&PathBuf> = files.iter().filter(|pb| pb.is_dir()).collect();
    directories.sort_unstable();
    let directories = directories;

    for (i, path) in directories.iter().enumerate() {
        if total_file_count > 1 {
            // Print newline if not the first thing to print
            if !file_names.is_empty() || i > 0 { writeln!(handle)?; }

            let pathname = path.to_string_lossy();
            writeln!(handle, "{pathname}:")?;
        }

        let dir = fs::read_dir(path)?;
        for f in dir {
            if let Some(filename) = f?.file_name().to_str() {
                write!(handle, "{} ", filename)?;
            }
        }

        writeln!(handle)?;
    }

    Ok(())
}
