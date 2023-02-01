use std::fs::File;
use std::io::prelude::*;

// this function will read the file and return the contents as a string
pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}
