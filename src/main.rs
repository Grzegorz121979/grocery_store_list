mod grocery;

use std::error::Error;
use std::io::{self, Write};

use grocery::{append_product_to_list, print_grocery_list, update_quantity, remove_product, clear_list, convert_csv_to_excel};

fn main() -> Result<(), Box<dyn Error>> {
    
    loop {
        let excel_path = "shop_list.xlsx";
        let file_path = "shop_list.csv";
        let mut user_input: String = String::new();
    
        print!("Enter a - append product, p - print grocery shop list, u - update quantity, r - remove, c - clear list, e - conver to xlsx: ");
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
            "r" => {
                match remove_product(file_path) {
                    Ok(()) => println!("Item remove"),
                    Err(e) => eprintln!("{}", e),
                }
            },
            "c" => {
                match clear_list(file_path) {
                    Ok(()) => println!("List cleared"),
                    Err(e) => eprintln!("{}", e),
                }
            },
            "e" => {
                match convert_csv_to_excel(file_path, excel_path) {
                    Ok(()) => println!("list convert"),
                    Err(e) => eprintln!("{}", e),
                }
            },
            _ => ()
        }
        
        let mut input = String::new();
        print!("Continue (y/n): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "n" {
            break;
        }
    }

    println!("Exit");

    Ok(())
}
