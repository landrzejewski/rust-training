use std::{env, process::exit};

pub fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn assert<T>(value: T, predicate: impl Fn(T) -> bool, show_errors: impl Fn()) {
    if !predicate(value) {
        show_errors();
        exit(0);
    }
}

pub fn min_length<T>(length: usize) -> impl Fn(&Vec<T>) -> bool {
    move |values| values.len() >= length
}

pub fn is_not_empty<T>(values: &Vec<T>) -> bool {
    !values.is_empty()
}

