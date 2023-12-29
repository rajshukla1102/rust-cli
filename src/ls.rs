use std::fs;
use colored::*;
pub fn run() {
    println!("Enter the path of the directory to list:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error reading line");
    let dir_path = input.trim();

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            println!("directory contents are:");
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("{}", file_name.green());
                }
            }
        }
        Err(err) => eprintln!("err listing directory {} {}", dir_path, err),
    }
}
