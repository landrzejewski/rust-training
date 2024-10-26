use std::{env, fs::File, io::{BufRead, BufReader}};

const ARG_PREFIX: &str = "-";

enum Mode {
    Default,
    WithNumbering { ignore_empty: bool }
}

impl Mode {

    fn from(options: &Vec<String>) -> Mode {
        if options.is_empty() {
            Mode::Default
        } else {
            match options[0].as_str() {
                "-n" => Mode::WithNumbering { ignore_empty: false },
                "-nb" => Mode::WithNumbering { ignore_empty: true },
                _ => Mode::Default
            }
        }
    }

}

fn main() {
    let (options, paths) = get_input();
    if paths.is_empty() {
        show_help();
        return;
    }
    let mode = Mode::from(&options);
    cat(&mode, &paths);
}

fn get_input() -> (Vec<String>, Vec<String>) {
    env::args().skip(1).partition(|arg| arg.starts_with(ARG_PREFIX))
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1, file2 ...");
    println!("options:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}

fn cat(mode: &Mode, paths: &Vec<String>) {
    let print_line = match mode {
        Mode::WithNumbering { ignore_empty: false } => print_with_numbering,
        Mode::WithNumbering { ignore_empty: true } => print_with_numbering_ignoring_empty,
        _ => print_line
    };
    for path in paths {
        let Ok(file) = File::open(path) else {
            eprintln!("Failed to open {path}");
            continue;
        };
        println!("File: {path}");
        let reader = BufReader::new(file);
        for (line_number, line) in reader.lines().enumerate() {
            let line_text = line.expect("Failed to read line");
            print_line(line_number + 1, &line_text);
        }
    }
}

fn print_line(_line_number: usize, line: &String) {
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
