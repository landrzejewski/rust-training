use std::env;

const SEPARATOR: &str = " ";

fn main() {
    let parameters = env::args()
        .skip(1)
        .collect::<Vec<_>>();
    println!("{}", parameters.join(SEPARATOR));
}
