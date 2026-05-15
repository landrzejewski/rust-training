use std::env;
use std::process::exit;

pub fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn assert<T>(value: T, predicate: impl Fn(T) -> bool, show_help: impl Fn()) {
    if !predicate(value) {
        show_help();
        exit(0);
    }
}

pub fn drop<T>(data: Vec<T>, count: usize) -> Vec<T> {
    data.into_iter().skip(count).collect()
}

pub fn min_length<T>(length: usize) -> impl Fn(&Vec<T>) -> bool {
    move |values| values.len() >= length
}

pub fn is_not_empty<T>(values: &Vec<T>) -> bool {
    !values.is_empty()
}
