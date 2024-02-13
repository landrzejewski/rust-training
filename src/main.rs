use std::env;

const SEPARATOR: &str = " ";

fn show_help() {
    println!("Usage:");
    println!("cat [args] file1 file2 ...");
    println!("Args:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers excluding blank lines");
}

fn main() {
    let parameters = env::args()
        .skip(1)
        .collect::<Vec<_>>();

}
