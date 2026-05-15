use crate::exercises::utils::{assert, drop, get_args, min_length};
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

fn show_help() {
    println!("Usage:");
    println!("grep text path1, path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn find_file_paths(path: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(path) {
        let Ok(entry) = entry else { continue };
        if entry.path().is_file() {
            files.push(entry.path().display().to_string());
        }
    }
    files
}

fn get_matching_lines(text: &String, file_path: &String) -> Vec<(usize, String)> {
    let Ok(file) = File::open(file_path) else {
        eprintln!("Unable to open file: {file_path}");
        return Vec::new();
    };
    let mut lines: Vec<(usize, String)> = Vec::new();
    let reader = BufReader::new(file);
    for entry in reader.lines().enumerate() {
        let (index, line) = entry;
        let Ok(line) = line else { break };
        if line.contains(text) {
            lines.push((index + 1, line));
        }
    }
    lines
}

fn print_matching_lines(matching_lines: &Vec<(usize, String)>) {
    for (line_number, line) in matching_lines {
        println!("[{:6}]: {}", line_number, line);
    }
}

fn grep(text: &String, paths: &Vec<String>) {
    for path in paths {
        let files = find_file_paths(path);
        for file in &files {
            println!("File: {file}");
            let matching_lines = get_matching_lines(text, file);
            print_matching_lines(&matching_lines);
        }
    }
}

pub fn run() {
    let args = get_args();
    assert(&args, min_length(2), show_help);
    let text = &args[0];
    let paths = drop(args.clone(), 1);
    grep(&text, &paths);
}
