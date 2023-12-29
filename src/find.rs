use std::fs;

pub fn run() {
    println!("Enter the path of the directory to search:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error reading line");
    let dir_path = input.trim();

    println!("Enter the keyword to search for:");
    let mut search = String::new();
    std::io::stdin().read_line(&mut search).expect("error reading line");
    let keyword = search.trim();

    match find_files(dir_path, keyword) {
        Ok(files) => {
            println!("Files containing '{}':", keyword);
            for file in files {
                println!("{}", file);
            }
        }
        Err(err) => eprintln!("Error finding files in '{}': {}", dir_path, err),
    }
}

fn find_files(dir_path: &str, keyword: &str) -> std::io::Result<Vec<String>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap_or_else(|_| String::new());
        if file_name.contains(keyword) {
            result.push(file_name);
        }
    }
    Ok(result)
}
