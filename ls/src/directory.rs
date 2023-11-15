use std::error::Error;
use std::io::{Stdout, BufWriter, Write};

use crate::cli::Cli;

pub enum Format {
    Inline,
}

pub struct Directory {
    name: String,
    items: Vec<String>,
}

impl Directory {
    pub fn new(name: String, items: Vec<String>) -> Self {
        Self { name, items }
    }

    pub fn write(&mut self, writer: &mut BufWriter<Stdout>, format: Format, write_name: bool, args: &Cli) -> Result<(), Box<dyn Error>> {
        let separator = match format {
            Format::Inline => " ",
        };

        if write_name {
            writeln!(writer, "{}", self.name)?;
        }

        self.items.sort_unstable();
        if !args.all && !args.almost_all {
            match self.items.split(|f| f.starts_with('.')).last() {
                Some(items) => self.items = items.to_vec(),
                None => return Ok(()),
            }
        }

        if args.all {
            write!(writer, ".{separator}")?;
            write!(writer, "..{separator}")?;
        }

        writeln!(writer, "{}", self.items.join(separator))?;

        Ok(())
    }
}
