use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use crate::exercises::util::{ensure_or_exit, get_args, min_length};

fn show_help() {
    println!("Usage:");
    println!("grep text path1, path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn find_file_paths(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let Ok(entry) = entry else { continue };
        // file_type() uses what walkdir already knows — path().is_file()
        // would issue another stat() and would follow symlinks.
        if entry.file_type().is_file() {
            files.push(entry.path().display().to_string());
        }
    }
    files
}

fn get_matching_lines(text: &str, file_path: &str) -> Vec<(usize, String)> {
    let Ok(file) = File::open(file_path) else {
        eprintln!("Unable to open file: {file_path}");
        return Vec::new();
    };
    let mut lines = Vec::new();
    for entry in BufReader::new(file).lines().enumerate() {
        let (index, line) = entry;
        // A single undecodable line must not silently truncate the file.
        let Ok(line) = line else {
            eprintln!("Skipping a non-UTF-8 line in {file_path}");
            continue;
        };
        if line.contains(text) {
            lines.push((index + 1, line));
        }
    }
    lines
}

fn print_matching_lines(matching_lines: &[(usize, String)]) {
    for (line_number, line) in matching_lines {
        println!("[{:6}]: {}", line_number, line);
    }
}

fn grep(text: &str, paths: &[String]) {
    for path in paths {
        let files = find_file_paths(path);
        for file in &files {
            let matching_lines = get_matching_lines(text, file);
            if !matching_lines.is_empty() {
                println!("{}", file);
                print_matching_lines(&matching_lines);
            }
        }
    }
}

pub fn run() {
    let args = get_args();
    ensure_or_exit(args.as_slice(), min_length(2), show_help);
    let text = args[0].as_str();
    let paths = &args[1..];
    grep(text, paths);
}
