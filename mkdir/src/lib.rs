use std::{error::Error, fs, path::PathBuf};

use args::Args;

pub mod args;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if args.directory.is_empty() {
        return Err("mkdir: Missing operand".into());
    }

    let mut errors: Vec<String> = vec![];

    for d in args.directory.iter() {
        if d.exists() {
            errors.push(format!("mkdir: cannot create directory '{}': File exists", d.display()));
            continue;
        }

        if let Some(p) = d.parent() {
            if !p.exists() && !args.parents {
                errors.push(format!("mkdir: cannot create directory '{}': No such file or directory", d.display()));
                continue;
            }
        }

        let create_fn = if args.parents { fs::create_dir_all::<&PathBuf> } else { fs::create_dir };

        if let Err(e) = create_fn(d) {
            errors.push(e.to_string());
        }
    }

    if !errors.is_empty() {
        let error_text = errors.join("\n");
        return Err(error_text.into());
    }

    Ok(())
}
