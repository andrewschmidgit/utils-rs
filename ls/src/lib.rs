use std::io::{BufWriter, Stdout, Write};
use std::path::Path;
use std::error::Error;

use bucket::{Bucket, Format};
use cli::Cli;

pub mod cli;
mod bucket;

pub fn run(args: Cli, writer: &mut BufWriter<Stdout>) -> Result<(), Box<dyn Error>>{
    let paths = match args.files.clone() {
        Some(f) => f,
        None => vec![".".into()],
    };

    let mut files: Vec<String> = vec![];
    let mut directories: Vec<Bucket> = Vec::with_capacity(paths.iter().filter(|p| p.is_dir()).count());

    for p in paths.iter() {
        if p.is_file() {
            files.push(format_path(p, &args))
        } else if p.is_dir() {
            let dir_contents = p.read_dir()?;

            let name = p.display().to_string() + ":";
            let items = dir_contents
                .filter_map(|f| {
                    match f {
                        Ok(f) => Some(format_path(&f.path(), &args)),
                        Err(_) => None,
                    }
                }).collect();

            let bucket = Bucket::new(name, items);

            directories.push(bucket);
        }
    }

    if !files.is_empty() {
        let mut files = files
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        files.sort_unstable();

        let line = files.join(" ");

        writeln!(writer, "{}", line)?;

        if !directories.is_empty() {
            writeln!(writer)?;
        }
    }

    let has_multiple_dirs = directories.len() > 1;
    for (i, mut b) in directories.into_iter().enumerate() {
        if i > 0 {
            writeln!(writer)?;
        }

        b.write(writer, Format::Inline, has_multiple_dirs)?;
    }

    Ok(())
}

fn format_path(path: &Path, _args: &Cli) -> String {
    match path.file_name() {
        Some(f) => f.to_owned().into_string().unwrap(),
        None => "".into(),
    }
}
