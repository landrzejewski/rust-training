use utils::{assert, get_args, is_not_empty, min_length};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

mod utils;

const ELEMENT_TYPE_SEPARATOR: &str = ",";

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

fn main() {
    let args = get_args();
    assert(&args, min_length(3), show_help);

    let regexp = Regex::new(&args[0]).expect("Invlid regex expression");

    let types = args[1]
        .split(ELEMENT_TYPE_SEPARATOR)
        .map(|element| element.trim())
        .filter_map(|element| ElementType::from(element))
        .collect::<Vec<_>>();
    assert(&types, is_not_empty, show_help);

    let paths = args.iter().skip(2).collect::<Vec<_>>();
    assert(&paths, is_not_empty, show_help);

    find(&regexp, &types, &paths).iter().for_each(|path| println!("{path}"));
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

fn find(regex: &Regex, types: &Vec<ElementType>, paths: &Vec<&String>) -> Vec<String> {
    let by_type = |entry: &DirEntry| {
        types.iter().any(|element_type| is_type_of(entry, element_type))
    };

    let by_name = |entry: &DirEntry| {
        let Some(file_name) = entry.file_name().to_str() else {
            return false;
        };
        regex.is_match(file_name)
    };

    let find_on_path = |path: &String| {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|element| element.ok())
            .filter(by_type)
            .filter(by_name)
            .map(|entry| entry.path().display().to_string())
    };

    println!("Searching...");

    paths.iter().fold(Vec::new(), |mut acc, path| {
        let matched_paths = find_on_path(path);
        acc.extend(matched_paths);
        acc
    })
}