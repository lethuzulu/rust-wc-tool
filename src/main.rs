use std::env::args;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
    // 1. get command line arguments

    let arguments: Vec<String> = args().collect();
    let file_string = &arguments[1];

    let file_path = Path::new(file_string);

    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();

    let mut temp_string = String::new();
    let num_bytes = file.read_to_string(&mut temp_string).unwrap();

    println!("Bytes: {:?}", num_bytes);

    let num_lines = temp_string.lines().count();

    println!("Lines: {:?}", num_lines);

    let num_words = temp_string.split(|c| c == '\n' || c == ' ').count();

    println!("Words: {:?}", num_words);

    let num_chars = temp_string.chars().count();
    println!("Chars: {:?}", num_words);

    
}
