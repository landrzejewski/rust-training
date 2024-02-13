use std::{env, process::exit};

const SEPARATOR: &str = " ";

fn show_help() {
    println!("Usage:");
    println!("cat [args] file1 file2 ...");
    println!("Args:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers excluding blank lines");
}

fn get_arguments() -> Vec<String> {
    env::args()
    .skip(1)
    .collect::<Vec<_>>()
}

#[derive(PartialEq)]
enum Mode {
    
    WithLineNumbers(bool),
    WithoutLineNumbers

}

fn get_mode(arguments: &Vec<String>) -> Mode {
    match arguments[0].as_str() {
        "-n" => Mode::WithLineNumbers(true),
        "-nb" => Mode::WithLineNumbers(false),
        _ => Mode::WithoutLineNumbers
    }
}

fn get_file_names(mode: &Mode, arguments: &Vec<String>) -> Vec<String> {
    if *mode == Mode::WithoutLineNumbers {
        arguments.clone()
    } else {
        arguments.iter()
        .skip(1)
        .cloned()
        .collect()
    }
}

fn main() {
    let arguments = get_arguments();
    if arguments.is_empty() {
        show_help();
        return;
    }
    let mode = get_mode(&arguments);
    let file_names = get_file_names(&mode, &arguments);
    

}
