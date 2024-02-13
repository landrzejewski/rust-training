use std::env;

fn show_help() {
    println!("Usage:");
    println!("wc file1 file2 ...");
}

fn get_arguments() -> Vec<String> {
    env::args()
    .skip(1)
    .collect::<Vec<_>>()
}

fn main() {
    let arguments = get_arguments();
}
