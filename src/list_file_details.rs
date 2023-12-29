use std::fs;
use colored::*;

pub fn run(){
    println!("Enter the path to list details: ");
    let mut input= String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    let input = input.trim();

    if input.len() < 2 {
        eprintln!("Please provide a path");
        return;
    }
    if !std::path::Path::new(input).exists() {
        eprintln!("Path does not exist");
        return;
    }
    let paths = fs::read_dir(input).expect("error");
    println!("path: {:?}",paths);
    for i in paths {
        let path = i.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_size = metadata.len();
        let file_size = if file_size == 0 {
            "0B".to_string()
        } else {
            format!("{}B", file_size)
        };
        let file_type = if metadata.is_dir() {
            "dir".to_string()
        } else {
            "file".to_string()
        };
        println!("{} {} {}", file_name.green(), file_size.blue(), file_type.yellow());
    }
}