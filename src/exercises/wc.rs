use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn show_help() {
    println!("Usage:");
    println!("wc file1 file2 ...");
}

fn get_arguments() -> Vec<String> {
    env::args().skip(1).collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Default)]
pub struct FileStats {
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

fn get_file_stats(file: &File) -> Result<FileStats, Box<dyn Error>> {
    let mut stats = FileStats::default();

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        stats.bytes += bytes;
        stats.chars += line.chars().count();
        stats.words += line.split_whitespace().count();
        stats.lines += 1;
        line.clear();
    }
    Ok(stats)
}

fn print_file_stats(file_name: &String, stats: &FileStats) {
    println!("File: {file_name}");
    println!("{:>8} bytes", stats.bytes);
    println!("{:>8} chars", stats.chars);
    println!("{:>8} words", stats.words);
    println!("{:>8} lines", stats.lines);
}

fn wc(file_names: &Vec<String>) {
    for file_name in file_names {
        match File::open(file_name) {
            Ok(file) => match get_file_stats(&file) {
                Ok(file_stats) => print_file_stats(file_name, &file_stats),
                Err(error) => eprintln!("Failed read from file {file_name} ({error})"),
            },
            Err(error) => eprintln!("Failed to open file {file_name} ({error})"),
        }
    }
}

pub fn run() {
    let arguments = get_arguments();
    if arguments.is_empty() {
        show_help();
        return;
    }
    wc(&arguments);
}
