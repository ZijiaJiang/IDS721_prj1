use std::fs::File;
use std::io::prelude::*;

// this function will take a vector of strings as the parameter and print each string in the vector
pub fn print_vec_str(vec: Vec<String>) {
    for item in vec {
        println!("    {}", item);
    }
}

// this function will read the csv file and return the columns titles as a vector of strings
pub fn read_csv(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let titles = lines.next().unwrap().split(',').collect::<Vec<&str>>();
    let mut titles_str = Vec::new();
    for title in titles {
        titles_str.push(title.to_string());
    }
    titles_str
}
