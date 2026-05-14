use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

const AGR_PREFIX: &str = "-";
const NUMBERING_ARG: &str = "-n";
const NUMBERING_IGNORE_EMPTY_ARG: &str = "-nb";

enum Mode {
    Default,
    Numbering { ignore_empty: bool },
}

impl From<&String> for Mode {
    fn from(value: &String) -> Self {
        match value.as_str() {
            NUMBERING_ARG => Mode::Numbering {
                ignore_empty: false,
            },
            NUMBERING_IGNORE_EMPTY_ARG => Mode::Numbering { ignore_empty: true },
            _ => Mode::Default,
        }
    }
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1 file2 ...");
    println!("options:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}

fn get_config() -> (Vec<String>, Vec<String>) {
    env::args()
        .skip(1)
        .partition(|arg| arg.starts_with(AGR_PREFIX))
}

fn cat(paths: &Vec<String>, mode: &Mode) {
    let printer: Printer = match mode {
        Mode::Numbering {
            ignore_empty: false,
        } => print_with_numbering,
        Mode::Numbering { ignore_empty: true } => print_with_numbering_ignoring_empty,
        _ => print,
    };

    for path in paths {
        let Ok(file) = File::open(path) else {
            eprintln!("Failed to open {path}");
            continue;
        };
        println!("File: {path}");
        BufReader::new(file)
            .lines()
            .enumerate()
            .for_each(|(index, line)| {
                printer(index + 1, &line.expect("Error reading line"));
            });
    }
}

type Printer = fn(usize, &String);

fn print(_line_numer: usize, line: &String) {
    println!("{line}");
}

fn print_with_numbering(line_numer: usize, line: &String) {
    println!("{:3}: \t{}", line_numer, line);
}

fn print_with_numbering_ignoring_empty(line_numer: usize, line: &String) {
    if line.is_empty() {
        println!();
    } else {
        print_with_numbering(line_numer, &line);
    }
}

pub fn run() {
    let (options, files) = get_config();
    if files.is_empty() {
        show_help();
        exit(0);
    }
    let mode = options.first().map(Mode::from).unwrap_or(Mode::Default);
    cat(&files, &mode);
}
