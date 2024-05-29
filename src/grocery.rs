use std::error::Error;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use csv::{WriterBuilder, Reader, Writer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        product: product.trim().to_string(),
        quantity: quantity.trim().parse()?,
    };

    append_to_file(path, &record)?;

    Ok(())
}

fn append_to_file(path: &str, record: &Record) -> Result<(), Box<dyn Error>> {
    let file: File = OpenOptions::new()
                                .append(true)
                                .open(path)?;
    
    let mut writer: Writer<File> = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(record)?;
    writer.flush()?;

    Ok(())
}

pub fn print_grocery_list(path: &str) -> Result<(), Box<dyn Error>> {
    match add_file_to_hashmap(path) {
        Ok(map) => {
            let mut vec: Vec<_> = map.iter().collect();
            vec.sort_by(|a, b| a.0.cmp(b.0));
            for (product, quantity) in vec {
                println!("{}: {}", product, quantity);
            } 
        },
        Err(e) => eprintln!("Error reading CSV file: {}", e)
    }

    Ok(())
}

pub fn add_file_to_hashmap(path: &str) -> Result<HashMap<String, u32>, Box<dyn Error>> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut reader = Reader::from_path(path)?;

    for row in reader.deserialize() {
        let record: Record = row?;
        map.insert(record.product, record.quantity);
    }

    Ok(map)
}
