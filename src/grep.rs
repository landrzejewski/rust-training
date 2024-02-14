use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use lib::get_arguments;
use walkdir::{DirEntry, WalkDir};
use utils::{assert, is_not_empty, min_length};

mod utils;
mod lib;

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

    let files = |path: &String| WalkDir::new(path)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(file_filter)
        .map(|entry| entry.path().display().to_string());


    println!("Searching...");
    
    let file_paths: Vec<String> = paths
        .iter()
        .fold(Vec::new(), |result, path| result.iter()
            .cloned()
            .chain(files(path).into_iter())
            .collect());

    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    file_paths.iter()
        .fold(&mut result, |result, file_path| { 
            result.insert(file_path.clone(), get_lines_with_text(text, file_path));
            result
        });

    return result
        .into_iter()
        .filter(|entry| !entry.1.is_empty())
        .collect();
}

fn main() {
    let arguments = get_arguments();
    assert(&arguments, min_length(2), show_help);

    let text = &arguments[0].clone();
    let paths = arguments.iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>();
    assert(&paths, is_not_empty(), show_help);

    for (key, lines) in grep(&text, &paths).into_iter() {
        println!("{key}");
        lines.iter().for_each(|line| println!("{line}"));
    };
}