pub mod cat;
pub mod ls;
pub mod find;
pub mod cp;
pub mod list_file_details;
pub mod searchinfile;
use std::io;

fn main() {
    loop {
        println!("Choose a command to execute:");
        println!("1. Cat");
        println!("2. Ls");
        println!("3. Find");
        println!("4. Cp");
        println!("5. List file details");
        println!("6. Search in file");
        println!("Type 'q' or 'quit' to exit.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "1" | "cat" => cat::run(),
            "2" | "ls" => ls::run(),
            "3" | "find" => find::run(),
            "4" | "cp" => cp::run(),
            "5" | "ls details" => list_file_details::run(),
            "6" | "search in file" => searchinfile::run(),
            "q" | "quit" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
