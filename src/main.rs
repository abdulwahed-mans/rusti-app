
use std::fs;

fn main() {
    let data_file_path = "cars.json";
    match fs::read_to_string(data_file_path) {
        Ok(data) => {
            println!("Contents of {}:", data_file_path);
            println!("{}", data);
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", data_file_path, e);
        }
    }
}
