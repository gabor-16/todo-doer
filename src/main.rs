use std::{fs::OpenOptions, io::{Read, Seek, SeekFrom, Write}};

fn main () {
    // file creation and opening
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("data.txt").unwrap();
    // file reading 
    let mut nom = String::new();
    file.read_to_string(&mut nom).expect("couldn't read file to string");

    // collectiong user input
    let option = std::env::args().nth(1).expect("couldn't read line");


    if option == "a" {
        // writing data to file
        // can use wrtite line instead of file.write
        let text = std::env::args().nth(2).expect("couldn't read line");
        file.write(text.as_bytes()).expect("couldn't write to file");
        file.write(b"\n").unwrap();

    } else if option == "r" {
        // reading data from file
        let mut content = String::new();
        file.seek(SeekFrom::Start(0)).unwrap(); //pointer reset
        file.read_to_string(&mut content).unwrap();
        println!("{}", content);

    } else if option == "d" {
        // deleting lines
    }
}