use regex::Regex;
use walkdir::{DirEntry, WalkDir};
use crate::exercises::util::{ensure_or_exit, get_args, is_not_empty, min_length};

const SEPARATOR: char = ',';

enum ElementType {
    Dir,
    File,
    Link
}

impl From<&str> for ElementType {
    fn from(value: &str) -> Self {
        match value {
            "dir" => ElementType::Dir,
            "link" => ElementType::Link,
            _ => ElementType::File
        }
    }
}

fn show_help() {
    println!("Usage:");
    println!("find regexp t1,t2,t3 path1 path2 ...");
    println!("options:");
    println!("  regexp - match/regular expression");
    println!("  types - one or many types separated by comma. Types: dir,file,link");
}

fn is_type_of(entry: &DirEntry, element_type: &ElementType) -> bool {
    let file_type = entry.file_type();
    match element_type {
        ElementType::Dir => file_type.is_dir(),
        ElementType::File => file_type.is_file(),
        ElementType::Link => file_type.is_symlink()
    }
}

fn find(regex: &Regex, types: &[ElementType], paths: &[String]) -> Vec<String> {
    let by_name = |entry: &DirEntry| regex.is_match(entry.file_name().to_str().unwrap_or_default());
    let by_type = |entry: &DirEntry| types.iter().any(|element_type: &ElementType| is_type_of(entry, element_type));
    let entry_to_string = |entry: DirEntry| entry.path().display().to_string();
    let find_on_path = |path: &String| {
        WalkDir::new(path)
            .into_iter()
            .flatten()
            .filter(by_name)
            .filter(by_type)
            .map(entry_to_string)
    };
    let accumulator = |mut acc: Vec<String>, path: &String| {
        acc.extend(find_on_path(path));
        acc
    };
    paths.iter().fold(vec![], accumulator)
}

pub fn run() {
    let args = get_args();
    ensure_or_exit(args.as_slice(), min_length(3), show_help);
    let regex = Regex::new(&args[0]).expect("Invalid regexp syntax");
    let types = args[1].split(SEPARATOR)
        .map(|element| element.trim())
        .map(ElementType::from)
        .collect::<Vec<_>>();
    let paths = &args[2..];
    ensure_or_exit(paths, is_not_empty, show_help);
    find(&regex, &types, paths)
        .iter()
        .for_each(|path| println!("{path}"));
}