use std::env;

const SEPARATOR: &str = " ";

pub fn run() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    println!("{}", args.join(SEPARATOR));
}