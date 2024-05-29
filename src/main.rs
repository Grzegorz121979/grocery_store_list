mod grocery;

use std::error::Error;
use std::io::{self, Write};

use grocery::{append_product_to_list, print_grocery_list, update_quantity};

fn main() -> Result<(), Box<dyn Error>> {
    
    loop {
        
        let file_path = "shop_list.csv";
        let mut user_input: String = String::new();
    
        print!("Enter a - append product, p - print grocery shop list, u - update quantity: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        let user_input: &str = user_input.trim();

        match user_input {
            "a" => {
                match append_product_to_list(file_path) {
                    Ok(()) => println!("Value append to the list"),
                    Err(e) => eprintln!("{}", e),
                }
            },
            "p" => {
                match print_grocery_list(&file_path) {
                    Ok(()) => println!("Grocery shop list printed"),
                    Err(e) => eprintln!("{}", e),
                }
            },
            "u" => {
                match update_quantity(file_path) {
                    Ok(()) => println!("Value from the list update"),
                    Err(e) => eprintln!("{}", e)
                }
            },
            _ => ()
        }
        break;
    }

    println!("Exit");

    Ok(())
}
