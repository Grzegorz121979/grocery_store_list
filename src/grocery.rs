use std::error::Error;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub product: String,
    pub quantity: u32,
}

pub fn append_product_to_list(path: &str) -> Result<(), Box<dyn Error>> {
    let mut product = String::new();
    let mut quantity = String::new();

    print!("Enter product: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut product)?;

    print!("Enter quantity: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut quantity)?;

    let record = Record {
        product: product.to_string(),
        quantity: quantity.trim().parse()?,
    };

    append_to_file(path, &record)?;

    Ok(())
}

fn append_to_file(path: &str, record: &Record) -> Result<(), Box<dyn Error>> {
    let file: File = OpenOptions::new()
                                .append(true)
                                .open(path)?;
    
    let mut writer: csv::Writer<File> = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(record)?;
    writer.flush()?;

    Ok(())
}
