use std::env;

const SEPARATOR: &str = " ";

pub fn run() {
    let text = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(SEPARATOR);
    println!("{text}");

    // env::args().skip(1).for_each(print_arg);
}

fn print_arg(arg: String) {
    print!("{arg} ")
}
