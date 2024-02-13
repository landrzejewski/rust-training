use std::env;

const SEPARATOR: &str = " ";

fn main() {
    let arguments = env::args()
        .skip(1)
        .collect::<Vec<_>>();
    println!("{}", arguments.join(SEPARATOR));
}
