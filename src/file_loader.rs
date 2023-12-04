use std::env;
use std::fs;

pub fn get_lines(file_name: &str) -> Vec<String> {
    let current_dir = env::current_dir()
        .expect("Failed to get current dir")
        .to_str()
        .expect("Failed to convert path to string")
        .to_owned();

    let file_path = current_dir + "/" + file_name;

    println!("loading path {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Failed to load file");

    contents
        .lines()
        .map(|s| s.to_string())
        .collect()
}
