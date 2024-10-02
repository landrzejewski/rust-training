use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use walkdir::{DirEntry, WalkDir};

fn show_help_and_exit() {
    show_help();
    exit(0);
}

fn drop<T>(vector: Vec<T>, count: usize) -> Vec<T> {
    vector.into_iter().skip(count).collect()
}

fn get_args() -> Vec<String> {
    drop(args().collect(), 1)
}

fn show_help() {
    println!("Usage:");
    println!("app text path1, path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn find_file_paths(path: &String) -> Vec<String> {
    let is_file = |entry: &DirEntry| entry.file_type().is_file();

    let mut files = Vec::new();

    WalkDir::new(path)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(is_file)
        .map(|entry| entry.path().display().to_string())
        .for_each(|path| files.push(path));

    files
}

fn print_lines_with_text(text: &String, file_paths: &Vec<String>) -> Result<(), Box<dyn Error>> {
    for file_path in file_paths {
        let mut lines = vec![];
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            if let Ok(current_line) = line {
                if current_line.contains(text) {
                    lines.push(format!("{:6}:\t{}", index + 1, current_line));
                }
            }
        }
        if !lines.is_empty() {
            println!("File: {file_path}:");
            lines.iter().for_each(|line| println!("{line}"));
        }
    }
    Ok(())
}

fn execute(text: &String, paths: &Vec<String>) -> Result<(), Box<dyn Error>>  {
    println!("Searching...");
    for path in paths {
        let file_paths = find_file_paths(path);
        print_lines_with_text(text, &file_paths)?;
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    if args.len() < 2 {
        show_help_and_exit()
    }
    let text = &args[0].clone();
    let paths = drop(args, 1);
    execute(&text, &paths)
}
