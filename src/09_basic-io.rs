use std::env;
use std::fs::*;
use std::io::{BufRead, BufReader, Write};

fn main() {
    // pobranie argumentów z linii komend
    let args: Vec<String> = env::args()
        .collect();
    println!("{:?}", args);

    // odczyt zmiennej środowiskowej
    if let Ok(path) = env::var("PATH") {
        println!("Path value: {}", path);
    }

    // odczyt zawartości pliku to String
    if let Ok(content) = read_to_string("notes.md") {
        for line in content.lines()
            .map(|line| line.to_uppercase()) {
            println!("{line}");
        }
    }

    // odczyt zawartości pliku linia po linii
    let file = File::open("notes.md").unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line {
            println!("{}: {}", index + 1, current_line)
        }
    }

    // utworzenie pliku
    // let test_file = File::create("test.txt");

    // zapis do pliku
    let test_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("test.txt");

    if let Ok(mut output_file) = test_file {
        writeln!(output_file, "Test value").expect("panic message");
    }
    
}