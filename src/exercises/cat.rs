use std::process::abort;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const ARG_PREFIX: &str = "-";
const NUMBERING_ARG: &str = "-n";
const NUMBERING_IGNORE_EMPTY_ARG: &str = "-nb";

enum Mode {
    Default,
    Numbering { empty: bool },
}

impl From<&String> for Mode {
    fn from(value: &String) -> Self {
        match value.as_str() {
            NUMBERING_ARG => Mode::Numbering { empty: false },
            NUMBERING_IGNORE_EMPTY_ARG => Mode::Numbering { empty: true },
            _ => Mode::Default,
        }
    }
}

pub fn run() {
    let (options, paths) = get_config();
    if paths.is_empty() {
        show_help();
        return;
    }
    
    let mode = options.get(0).map(Mode::from).unwrap_or(Mode::Default);
    cat(&mode, &paths);
}

fn get_config() -> (Vec<String>, Vec<String>) {
    env::args()
        .skip(1)
        .partition(|arg| arg.starts_with(ARG_PREFIX))
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1 file2 ...");
    println!("options:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}

fn cat(mode: &Mode, paths: &Vec<String>) {
    let print_line: Printer = match mode {
        Mode::Numbering { empty: false } => print_with_numbering,
        Mode::Numbering { empty: true } => print_with_numbering_ignoring_empty,
        _ => print,
    };

    for path in paths {
        let Ok(file) = File::open(path) else {
            eprintln!("Filed to open the file: {path}");
            continue;
        };
        println!("File: {path}");
        BufReader::new(file)
            .lines()
            .enumerate()
            .for_each(|(index, line)| print_line(index + 1, &line.expect("Failed to read line")));
    }
}

fn print_line(line_number: usize, line: &String, printer: &Printer) {
    printer(line_number, line)
}

type Printer = fn(usize, &String);

// type Distance = usize;

fn print(_line_number: usize, line: &String) {
    println!("{line}");
}

fn print_with_numbering(line_number: usize, line: &String) {
    println!("{:3}:\t{}", line_number, line);
}

fn print_with_numbering_ignoring_empty(line_number: usize, line: &String) {
    if line.is_empty() {
        println!();
    } else {
        print_with_numbering(line_number, line);
    }
}
