use std::fs;

pub fn run() {
    println!("Enter the path of the file to display:");
    let mut file_path = String::new();
    std::io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim();

    match fs::read_to_string(file_path) {
        Ok(content) => println!("File content:\n{}", content),
        Err(err) => eprintln!("Error reading file '{}': {}", file_path, err),
    }
}
