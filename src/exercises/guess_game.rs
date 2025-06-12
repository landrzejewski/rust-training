use io::stdin;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number!");
    let number = rand::rng().random_range(1..=10);
    loop {
        println!("Please input number: ");
        let mut provided_number = String::new();
        stdin()
            .read_line(&mut provided_number)
            .expect("Read line failed");

        let Ok(guess) = provided_number.trim().parse::<usize>() else {
            continue;
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
