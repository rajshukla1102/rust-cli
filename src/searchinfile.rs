use colored::*;
use std::fs;
use std::io;

pub fn run() {
    println!("Enter the directory path to search:");
    let mut directory_path = String::new();
    io::stdin().read_line(&mut directory_path).expect("error");
    let directory_path = directory_path.trim();

    println!("Enter the text to search for:");
    let mut search_text = String::new();
    io::stdin().read_line(&mut search_text).expect("error");
    let search_text = search_text.trim();

    if directory_path.is_empty()||(!std::path::Path::new(directory_path).exists()) {
        return;
    }
    
    let entries = match fs::read_dir(directory_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("err: {}", err);
            return;
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if file_path.is_file() {
                match search_text_in_file(&file_path, search_text) {
                    Ok(found_lines) => {
                        if !found_lines.is_empty() {
                            println!("File: {}", file_path.to_string_lossy().green());
                            for line in found_lines {
                                println!("  Found at line {}", line);
                            }
                            println!();
                        }
                    }
                    Err(err) => eprintln!("err {}: {}", file_path.to_string_lossy(), err),
                }
            }
        }
    }
}   
fn search_text_in_file(file_path: &std::path::Path, search_text: &str) -> Result<Vec<usize>, std::io::Error> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => return Err(err),
    };

    let mut found_lines = Vec::new();

    for (line_number, line) in file_content.lines().enumerate() {
        if line.contains(search_text) {
            found_lines.push(line_number);
        }
    }

    Ok(found_lines)
}