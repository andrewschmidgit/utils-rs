use std::error::Error;
use std::io::{Stdout, BufWriter, Write};

pub enum Format {
    Inline,
}

pub struct Bucket {
    name: String,
    items: Vec<String>,
}

impl Bucket {
    pub fn new(name: String, items: Vec<String>) -> Self {
        Self { name, items }
    }

    pub fn write(&mut self, writer: &mut BufWriter<Stdout>, format: Format, write_name: bool) -> Result<(), Box<dyn Error>> {
        let separator = match format {
            Format::Inline => " ",
        };

        if write_name {
            writeln!(writer, "{}", self.name)?;
        }

        self.items.sort_unstable();
        writeln!(writer, "{}", self.items.join(separator))?;

        Ok(())
    }
}
