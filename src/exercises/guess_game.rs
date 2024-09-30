use rand::Rng;
use std::cmp::Ordering::{Greater, Less};
use std::io;

pub fn run() {
    println!("Guess the number!");
    let number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input number: ");
        let mut provided_number = String::new();

        io::stdin()
            .read_line(&mut provided_number)
            .expect("Read line failed");

        let guess = match provided_number.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Incorrect number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&number) {
            Less => println!("Too small"),
            Greater => println!("Too big"),
            _ => {
                println!("You won!");
                break;
            }
        }
    }
}
