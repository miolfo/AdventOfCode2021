use std::{
    fs::File,
    io::{prelude::*, BufReader},
};


pub fn read_resource(file_name: &str) -> Vec<String> {
    let mut file_path: String = "src/resources/".to_owned();
    file_path.push_str(file_name);
    println!("reading {}", file_path);
    let file = File::open(file_path).expect("Unable to read file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}