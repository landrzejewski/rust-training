use std::env;
use std::fs::*;
use std::io::{BufRead, BufReader, Write};

pub fn run() {
    // retrieving arguments from the command line
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // reading the environment variable
    if let Ok(path) = env::var("PATH") {
        println!("Path value: {}", path);
    }

    // reading the contents of the file to String
    if let Ok(content) = read_to_string("exercises/notes.md") {
        for line in content.lines().into_iter().map(|line| line.to_uppercase()) {
            println!("{line}");
        }
    }

    // reading the contents of the file line by line
    let file = File::open("notes.md").unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line {
            println!("{}: {}", index + 1, current_line)
        }
    }

    // file creation

    // let test_file = File::create("tests.txt");

    // writing to file
    let test_file = OpenOptions::new().read(true).write(true).create(true).append(true).open("tests.txt");

    if let Ok(mut output_file) = test_file {
        writeln!(output_file, "Test value").expect("panic message");
    }
}
