use std::{fs::OpenOptions, io::Read};

fn main () {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("data.txt").unwrap();
    let mut nom = String::new();
    file.read_to_string(&mut nom).expect("couldn't read file to string");

}