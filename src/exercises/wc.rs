use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[derive(Default)]
struct FileStats {
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

impl Display for FileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _ = writeln!(f, "{:>8} bytes", self.bytes);
        _ = writeln!(f, "{:>8} chars", self.chars);
        _ = writeln!(f, "{:>8} words", self.words);
        _ = writeln!(f, "{:>8} lines", self.lines);
        Ok(())
    }
}

fn show_help() {
    println!("Usage:");
    println!("wc file1 file2 ...");
}

fn wc(paths: &Vec<String>) {
    for path in paths {
        match File::open(path) {
            Ok(file) => match get_file_stats(&file) {
                Ok(stats) => println!("{stats}"),
                Err(error) => eprintln!("Error: {}", error),
            },
            Err(error) => eprintln!("Error opening file '{}': {}", path, error),
        }
    }
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
    }
    Ok(stats)
}


pub fn run() {
    let paths = env::args().skip(1).collect::<Vec<String>>();
    if paths.is_empty() {
        show_help();
        exit(0);
    }
    wc(&paths);
}
