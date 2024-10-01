use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Mode {
    WithNumbering(bool),
    Plain,
}

fn get_mode(arguments: &Vec<String>) -> Mode {
    match arguments[0].as_str() {
        "-n" => Mode::WithNumbering(true),
        "-nb" => Mode::WithNumbering(false),
        _ => Mode::Plain,
    }
}

fn get_file_names(mode: &Mode, arguments: &Vec<String>) -> Vec<String> {
    match mode {
        Mode::Plain => arguments.clone(),
        _ => arguments[1..].to_vec()
    }
}

fn show_help() {
    println!("Usage:");
    println!("cat [args] file1 file2 ...");
    println!("Args:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers excluding blank lines");
}

fn get_arguments() -> Vec<String> {
    env::args().skip(1).collect()
}

fn print_line(_line_number: &mut i32, line: &String) {
    println!("{}", line)
}

fn print_with_line_numbers(line_number: &mut i32, line: &String) {
    *line_number += 1;
    println!("{:6}:\t{}", line_number, line);
}

fn print_with_line_numbers_without_empty_lines(line_number: &mut i32, line: &String) {
    if line.is_empty() {
        println!();
    } else {
        print_with_line_numbers(line_number, line);
    }
}

fn print_file_lines(mode: &Mode, file: &File) -> Result<(), Box<dyn Error>> {
    let print = match mode {
        Mode::WithNumbering(true) => print_with_line_numbers,
        Mode::WithNumbering(false) => print_with_line_numbers_without_empty_lines,
        Mode::Plain => print_line,
    };

    let mut line_number = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        print(&mut line_number, &line?);
    }
    Ok(())
}

fn cat(mode: &Mode, file_names: &Vec<String>) -> Result<(), Box<dyn Error>> {
    for file_name in file_names {
        let file = File::open(file_name)?;
        println!("File: {file_name}");
        print_file_lines(&mode, &file)?;
        println!();
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let arguments = get_arguments();
    if arguments.is_empty() {
        show_help();
        return Ok(());
    }
    let mode = get_mode(&arguments);
    let file_names = get_file_names(&mode, &arguments);
    cat(&mode, &file_names)
}
