use std::fs::{read_to_string, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main () {
    // data file name/location
    let data_file: &str = "todo-doer_data.txt";
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
            let lines = read_all_lines(data_file);

            if lines.is_empty() {
                println!("Todo list is empty.");
            } else {
                for (index, line) in lines.iter().enumerate() {
                    println!("{}.{}", index + 1, line);
                }
            }
            }

        "-s" => {
            // reading selected line of file
            let line_number_str= std::env::args().nth(2).expect("couldn't read line number");
            let line_number = match line_number_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: Line number must be an integer");
                    return;
                }
            };
            println!("{:?}", read_line_from_file(data_file, line_number));
            }

        "--clear-list" => {
            // deleting the data file and creating a new empty one
            OpenOptions::new().write(true).truncate(true).open(data_file).unwrap();
        }

        "-d" => {
            // deleting lines from data file
            let line_for_deletion_str = std::env::args().nth(2).expect("could't read line for deletion");
            let line_for_deletion_num: usize = match line_for_deletion_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: Line number must be an iteger");
                    return;
                }
            };
            // reads file and turns lines into vec
            // FIXME: if possible make this not need 2 for loops and a lot of space
            let lines = read_all_lines(data_file);
            if lines.is_empty() {
                println!("Todo list is empty.");
            } else {
                let mut new_file = Vec::new();
                for (index, line) in lines.iter().enumerate() {
                    if index == (line_for_deletion_num -1) {
                        let new_line = line.replace(line, "");
                        new_file.push(new_line.to_string());
                    } else {
                        new_file.push(line.to_string());
                    }
                }
                // turns vec into string and deletes epmty strings
                let mut result = String::new();
                for s in new_file {
                    if !(s == "") {
                        result.push_str(&s);
                        result.push_str("\n");
                    }
                }

                let mut replaced_file = OpenOptions::new().write(true).truncate(true).open(data_file).unwrap();
                replaced_file.write_all(result.as_bytes()).unwrap();
            }
        }

        "--help" | "-h" | _ => {
            // help option
            println!("This is a help message");
            println!("usage: cargo run -- [option] [text]")
        }
    }
}

// finction for reading lines of data file
fn read_line_from_file(filename: &str, line_number: usize) -> String{
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    } 
    if line_number == 0 || line_number > result.len() {
        return format!("Error: Line number {} is out of range.", line_number);
    } else {
        return result[line_number - 1].clone();        
    }    
}
// reads data file into memory
fn read_all_lines(filename: &str) -> Vec<String> {
    BufReader::new(OpenOptions::new().read(true).open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}
