use std::error::Error;
use std::io::{self, Write};

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

    Ok(())
}

/* 

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub product: String,
    pub quantity: u32,
}

pub fn append_value_to_list(path: &str) -> Result<(), Box<dyn Error>> {
    let mut product: String = String::new();
    let mut quantity: String = String::new();

    print!("Enter product: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut product)?;
    print!("Enter quantity: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut quantity)?;

    let record: Record = Record {
        product: product.trim().to_string(),
        quantity: quantity.trim().parse()?,
    };

    append_to_csv(path, &record)?;

    Ok(())
}

pub fn append_to_csv(path: &str, record: &Record) -> Result<(), Box<dyn Error>> {
    let file: File = OpenOptions::new()
                                .append(true)
                                .open(path)?;
                    
    let mut writer: Writer<File> = WriterBuilder::new().has_headers(false).from_writer(file);
                
    writer.serialize(record)?;
    writer.flush()?;
                
    Ok(())
}
*/