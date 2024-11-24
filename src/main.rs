use std::fs::{self, read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

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
    match option.as_str() {
        
        "-a" => {
            // writing data to file
            let text = std::env::args().nth(2).expect("couldn't read text for file");
            file.write(text.as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();            
        }

        "-r" => {
            // reading data from file
            let reader = BufReader::new(file);
            let mut count = 0;
            for line in reader.lines() {
                count = count + 1;
                let line = line.unwrap();
                println!("{}.{}", count, line);
                }
            }

        "-s" => {
            // reading selected line of file
            let line_number_str= std::env::args().nth(2).expect("couldn't read line number");
            let line_number = line_number_str.parse().unwrap();
            println!("{:?}", read_line(data_file, line_number));
            }

        "--clear-list" => {
            // deleting the data file and creating a new empty one
            fs::remove_file(data_file).unwrap();
            File::create(data_file).unwrap();
            }

        "-d" => {
            // deleting lines from data file
        }
        // no option/ not supported
        _ => println!("No option selected")
    }
}

// finction for reading lines of data file
fn read_line(filename: &str, line_number: usize) -> String{
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    } 
    return result[line_number - 1].clone();
}
