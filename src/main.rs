use std::{env, process::exit};

use regex::Regex;
use walkdir::{DirEntry, WalkDir};

const ELEMENT_TYPE_SEPARATOR: &str = ",";

fn show_help() {
    println!("Usage:");
    println!("find regexp t1,t2,t3 path1 path2 ...");
    println!("Args:");
    println!("  regexp - match/regular expression");
    println!("  types - one or many types separated by comma. Types: dir,file,link");
}

fn get_arguments() -> Vec<String> {
    env::args()
    .skip(1)
    .collect::<Vec<_>>()
}

enum ElementType {
    
    Dir,
    File,
    Link

}

impl ElementType {
    
    fn from(text: &str) -> Option<Self> {
        match text {
            "dir" => Some(ElementType::Dir),
            "file" => Some(ElementType::File),
            "link" => Some(ElementType::Link),
            _ => None
        }
    }

}

fn assert<T>(value: T, predicate: impl Fn(T) -> bool) {
    if !predicate(value) {
        show_help();
        exit(0);
    }
}

fn min_length<T>(length: usize) -> impl Fn(&Vec<T>) -> bool {
    move |values: &Vec<T>| values.len() >= length
} 

fn is_not_empty<T>() -> impl Fn(&Vec<T>) -> bool {
    |values: &Vec<T>| !values.is_empty()
}

fn is_type_of(entry: &DirEntry, element_type: &ElementType) -> bool {
    match element_type {
        ElementType::Dir => entry.file_type().is_dir(),
        ElementType::File => entry.file_type().is_file(),
        ElementType::Link => entry.file_type().is_symlink(),
    }
}

fn find(regex: &Regex, element_types: &Vec<ElementType>, paths: &Vec<String>) -> Vec<String> {
    let element_type_filter = |entry: &DirEntry| element_types
        .iter()
        .any(|element_type| is_type_of(entry, element_type));

    let name_filter = |entry: &DirEntry| {
        let Some(file_name) = entry.file_name().to_str() else {
            return false;
        };
        regex.is_match(file_name)
    };

    let find_on_path = |path: &String| WalkDir::new(path)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(element_type_filter)
        .filter(name_filter)
        .map(|entry| entry.path().display().to_string());;


    println!("Searching...");
    
    let mut result = Vec::new();

    paths.iter()
        .fold(&mut result, |result, path| { 
            result.extend(find_on_path(path));
            result
        });

    return result;
}

fn main() {
    let arguments = get_arguments();
    assert(&arguments, min_length(3));

    let regex = Regex::new(&arguments[0])
        .expect("Invalid regexp expression");
    let element_types = arguments[1]
        .split(ELEMENT_TYPE_SEPARATOR)
        .map(|element| element.trim())
        .filter_map(|element| ElementType::from(element))
        .collect::<Vec<_>>();
    assert(&element_types, is_not_empty());
    let paths = arguments.iter()
        .skip(2)
        .cloned()
        .collect::<Vec<_>>();
    assert(&paths, is_not_empty());

    find(&regex, &element_types, &paths)
        .iter()
        .for_each(|path| println!("{path}"));
}