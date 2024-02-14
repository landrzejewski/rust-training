use std::{process::exit};

pub fn assert<T>(value: T, predicate: impl Fn(T) -> bool, show_info: impl Fn()) {
    if !predicate(value) {
        show_info();
        exit(0);
    }
}

pub fn min_length<T>(length: usize) -> impl Fn(&Vec<T>) -> bool {
    move |values: &Vec<T>| values.len() >= length
} 

pub fn is_not_empty<T>() -> impl Fn(&Vec<T>) -> bool {
    |values: &Vec<T>| !values.is_empty()
}
