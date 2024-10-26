use std::env;

fn main() {
    // const SEPARATOR: &str = " ";
    // let args: Vec<String> = env::args().collect();

    /*
    let echo = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(SEPARATOR);
    println!("{echo}");
    */

    // env::args().skip(1).for_each(|arg| print!("{arg} "));

    env::args().skip(1).for_each(print_arg);
}

fn print_arg(arg: String) {
    print!("{arg} ")
}
