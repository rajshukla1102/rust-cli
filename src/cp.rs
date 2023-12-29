use std::fs;
use std::path::Path;

pub fn run() {
    println!("Enter the source path with copy details:");
    let mut source = String::new();
    std::io::stdin()
        .read_line(&mut source)
        .expect("Error reading line");
    let source = source.trim();

    println!("Enter the destination path with copy details:");
    let mut target = String::new();
    std::io::stdin()
        .read_line(&mut target)
        .expect("Error reading line");
    let target = target.trim();

    if !Path::new(source).exists() {
        eprintln!("Source path does not exist");
        return;
    }

    if !Path::new(target).exists() {
        println!("target: {}", target);
        let target_i = target.split("\\").collect::<Vec<&str>>();
        let target_i = target_i[..target_i.len() - 1].join("\\");
        println!("target_i: {:?}", target_i);
        match fs::create_dir_all(&target_i) {
            Ok(_) => println!("directory created successfully"),
            Err(err) => {
                eprintln!("err {}", err);
                return;
            }
        }
    }

    match fs::copy(source, target) {
        Ok(_) => {
            println!("Copied successfully!");
        }
        Err(err) => eprintln!("Error copying file from {} to {}: {}", source, target, err),
    }
}
