use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use walkdir::{DirEntry, WalkDir};
use crate::exercises::util::{ensure_or_exit, get_args, is_not_empty, min_length};

fn show_help() {
    println!("Usage:");
    println!("app text path1 path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn get_lines_with_text(text: &str, file_path: &str) -> Vec<String> {
    let mut lines = vec![];
    let Ok(file) = File::open(file_path) else {
        return Vec::new();
    };
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line
            && current_line.contains(text)
        {
            lines.push(format!("{:6}:\t{}", index + 1, current_line));
        }
    }
    lines
}

fn grep(text: &str, paths: &[String]) -> HashMap<String, Vec<String>> {
    let by_file = |entry: &DirEntry| entry.file_type().is_file();
    let entry_to_string = |entry: DirEntry| entry.path().display().to_string();

    let to_files = |path: &String| {
        WalkDir::new(path)
        .into_iter()
        .flatten() // skips unreadable entries (drops the Err results)
        .filter(by_file)
        .map(entry_to_string)
    };

    println!("Searching...");
    paths
        .iter()
        .flat_map(to_files)
        .map(|path| {
            let lines = get_lines_with_text(text, &path);
            (path, lines)
        })
        .filter(|entry| !entry.1.is_empty())
        .collect()
}

pub fn run() {
    let args = get_args();
    ensure_or_exit(args.as_slice(), min_length(2), show_help);

    let text = args[0].as_str();
    let paths = &args[1..];
    ensure_or_exit(paths, is_not_empty, show_help);

    for (file, lines) in grep(text, paths) {
        println!("{file}");
        lines.iter().for_each(|line| println!("{line}"));
    }
}
