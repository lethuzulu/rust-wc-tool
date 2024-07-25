use std::env::args;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

fn main() {
    //1. get command line arguments

    let arguments: Vec<String> = args().collect();
    let file_string = &arguments[1];

    let file_path = Path::new(file_string);

    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();

    let mut temp_buf: Vec<u8> = vec![];

    let num_bytes = file.read_to_end(&mut temp_buf).unwrap();

     println!("{:?}", num_bytes);
}
