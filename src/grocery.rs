use std::error::Error;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use csv::{WriterBuilder, Reader, Writer};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub product: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub product: String,
    pub quantity: String,
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

fn append_header(path: &str, header: &Header) -> Result<(), Box<dyn Error>> {
    let file: File = OpenOptions::new()
                                .append(true)
                                .open(path)?;
    
    let mut writer: Writer<File> = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.serialize(header)?;
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

pub fn update_quantity(path: &str) -> Result<(), Box<dyn Error>> {
    let mut map = add_file_to_hashmap(path)?;

    let mut product = String::new();
    let mut new_quantity = String::new();

    print!("Enter product: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut product)?;
    let product = product.trim();

    print!("Enter new quantity: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut new_quantity)?;
    let new_quantity: u32 = new_quantity.trim().parse()?;

    match map.get_mut(product) {
        Some(quantity) => {
            *quantity = new_quantity;
        },
        None => {
            println!("Key {} not found", product);
        }
    };

    match add_file_to_csv(path, map) {
        Ok(()) => println!("Product were uptated"),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

pub fn remove_product(path: &str) -> Result<(), Box<dyn Error>> {
    let mut name = String::new();
    println!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    let mut new_map: HashMap<String, u32> = HashMap::new();
    match add_file_to_hashmap(path) {
        Ok(map) => {
            for (key, value) in map {
                if key != name {
                   new_map.insert(key, value);
                }
            }
        },
        Err(e) => eprintln!("{}", e)
    }

    add_file_to_csv(path, new_map)?;
    
    Ok(())
}

pub fn clear_list(path: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(path)?;
        
    file.set_len(0)?;

    let header = Header {
        product: String::from("product"),
        quantity: String::from("quantity"),
    };

    append_header(path, &header)?;

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

pub fn add_file_to_csv(path: &str, map: HashMap<String, u32>) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_path(path)?;

    for (product, quantity) in map {
        let record = Record {
            product: product.clone(),
            quantity: quantity.clone(),
        };

        writer.serialize(record)?;
    }

    writer.flush()?;

    Ok(())
}
