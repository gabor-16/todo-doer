use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, Read, Seek, SeekFrom, Write}};

fn main () {
    // file creation and opening
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("data.txt")
        .expect("couldn't find file");
    
    // collectiong user input
    let option = std::env::args().nth(1).expect("couldn't read argument");
    // options for the program
    // possible change: replace if for match
    if option == "-a" {
        // writing data to file
        let text = std::env::args().nth(2).expect("couldn't read text for file");
        write!(file, "{}.", (line_count()+1)).unwrap();
        writeln!(file, "{}", text).unwrap();

    } else if option == "-r" {
        // reading data from file
        let mut content = String::new();
        file.seek(SeekFrom::Start(0)).unwrap(); //pointer reset
        file.read_to_string(&mut content).unwrap();
        println!("{}", content);

    } else if option == "-d" {
        // deleting lines
    } else {
        println!("No option selected");
    }
}

// function for counting ammount of lines in data file
// return i8 probably enough for a todo app
fn line_count() -> i8 {
    let file_lines = BufReader::new(File::open("data.txt").expect("Unable to open file"));
    let mut count  = 0;
    
    for _ in file_lines.lines() {
        count = count + 1;
    }
    return count;
}