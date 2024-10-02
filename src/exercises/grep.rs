use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::exercises::utils::{assert, get_args, is_not_empty, min_length};
use walkdir::{DirEntry, WalkDir};

fn show_help() {
    println!("Usage:");
    println!("app text path1 path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn get_lines_with_text(text: &String, file_path: &String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let Ok(file) = File::open(file_path) else {
        return Vec::new();
    };
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line {
            if current_line.contains(text) {
                lines.push(format!("{:6}:\t{}", index + 1, current_line));
            }
        }
    }
    lines
}

fn grep(text: &String, paths: &Vec<String>) -> HashMap<String, Vec<String>> {
    let file_filter = |entry: &DirEntry| entry.file_type().is_file();

    let files = |path: &String| {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|result| result.ok())
            .filter(file_filter)
            .map(|entry| entry.path().display().to_string())
    };

    println!("Searching...");
    paths.into_iter()
        .map(files)
        .flat_map(|files| files)
        .map(|path| (path.clone(), get_lines_with_text(text, &path)))
        .filter(|entry| !entry.1.is_empty())
        .collect()
}

pub fn run() {
    let args = get_args();
    assert(&args, min_length(2), show_help);

    let text = &args[0].clone();
    let paths = args.iter().skip(1).cloned().collect::<Vec<_>>();
    assert(&paths, is_not_empty(), show_help);

    for (file, lines) in grep(&text, &paths).into_iter() {
        println!("{file}");
        lines.iter().for_each(|line| println!("{line}"));
    }
}
