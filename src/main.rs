use std::fs::{read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

fn main () {
    // data file name/location
    let data_file: &str = "todo_doer_data.txt";
    // file creation and opening
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(data_file)
        .expect("couldn't find file");

    // collectiong user input
    let option = std::env::args().nth(1).expect("couldn't read argument");
    
    // options for the program
    // possible change: replace if for match
    if option == "-a" {
        // writing data to file
        let text = std::env::args().nth(2).expect("couldn't read text for file");
        write!(file, "{}.", (line_count(data_file)+1)).unwrap();
        writeln!(file, "{}", text).unwrap();

    } else if option == "-r" {
        // reading data from file
        let mut content = String::new();
        file.seek(SeekFrom::Start(0)).unwrap(); //pointer reset
        file.read_to_string(&mut content).unwrap();
        println!("{}", content);

    } else if option == "-s" {
        // reading selected line of file
        let line_number_str= std::env::args().nth(2).expect("couldn't read line number");
        let line_number = line_number_str.parse().unwrap();
        println!("{:?}", read_file(data_file, line_number));
    } else if option == "-d" {
        // deleting lines
    
    } else {
        println!("No option selected");
    }
}

// finction for reading lines of data file
fn read_file(filename: &str, line_number: usize) -> String{
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    } 
    return result[line_number - 1].clone();
}

// function for counting ammount of lines in data file
// return i8 probably enough for a todo app
fn line_count(data_file: &str) -> i8 {
    let file_lines = BufReader::new(File::open(data_file).expect("Unable to open file"));
    let mut count  = 0;
    
    for _ in file_lines.lines() {
        count = count + 1;
    }
    return count;
}