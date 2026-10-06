use std::env;

const SEPARATOR: &str = " ";

pub fn run() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    for (index, arg) in args.iter().enumerate() {
        print!("{arg}");
        if index < args.len() - 1 {
            print!("{SEPARATOR}");
        }
    }
    println!();
    //println!("{}", args.join(SEPARATOR));
}