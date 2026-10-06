use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

const ARG_PREFIX: &str = "-";
const NUMBERING_ARG: &str = "-n";
const NUMBERING_IGNORE_EMPTY_ARG: &str = "-nb";

enum Mode {
    Normal,
    Numbering { ignore_empty: bool },
}

impl From<&String> for Mode {
    fn from(arg: &String) -> Self {
        match arg.as_str() {
            NUMBERING_ARG => Mode::Numbering { ignore_empty: false },
            NUMBERING_IGNORE_EMPTY_ARG => Mode::Numbering { ignore_empty: true },
            _ => Mode::Normal,
        }
    }
}

fn get_config() -> (Mode, Vec<String>) {
    let config: (Vec<String>, Vec<String>) = args().skip(1)
        .partition(|arg| arg.starts_with(ARG_PREFIX));
    let (options, files) = config;
    if files.is_empty() {
        show_help();
        exit(0);
    }
    let mode= options.first().map(Mode::from).unwrap_or(Mode::Normal);
    (mode, files)
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1 file2 ...");
    println!("options:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}

fn print(_line_number: usize, line: &str) {
    println!("{line}");
}

fn print_with_numbering(line_number: usize, line: &str) {
    println!("{:3}: \t{}", line_number, line);
}

fn print_with_numbering_ignore_empty(line_number: usize, line: &str) {
    if line.is_empty() {
        println!("{:3} \t{}", "", line);
    } else {
        print_with_numbering(line_number, line);
    }

}

type Printer = fn(usize, &str);

fn cat((mode, filenames): (Mode, Vec<String>)) {
    let printer: Printer = match mode {
        Mode::Normal => print,
        Mode::Numbering { ignore_empty: false } => print_with_numbering,
        Mode::Numbering { ignore_empty:true } => print_with_numbering_ignore_empty
    };
    for filename in &filenames {
        let Ok(file) = File::open(filename) else {
            eprintln!("error: unable to open file");
            continue;
        };
        println!("File: {}", filename);
        let mut  line_number = 0;
        for next_line in BufReader::new(file).lines() {
            let Ok(line) = next_line else {
                eprintln!("error: unable to open file");
                break;
            };
            line_number += 1;
            printer(line_number, &line)
        }
    }

}

pub fn run() {
    cat( get_config());
}