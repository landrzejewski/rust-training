use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default)]
struct FileStats {
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

fn main() {
    let args = get_args();
    if args.is_empty() {
        show_help();
        return;
    }
    wc(&args);
}

fn show_help() {
    println!("Usage:");
    println!("wc file1 file2 ...");
}

fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn wc(paths: &Vec<String>) {
    for path in paths {
        match File::open(path) {
            Ok(file) => match get_file_stats(&file) {
                Ok(file_stats) => print_file_stats(path, &file_stats),
                Err(error) => eprintln!("Failed read from file {path} ({error})"),
            },
            Err(error) => eprintln!("Failed to open file {path} ({error})"),
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