use std::env;

const SEPARATOR: &str = " ";

pub fn run() {
    let echo = env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(SEPARATOR);
    println!("{echo}");
}
