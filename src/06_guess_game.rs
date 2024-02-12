use std::cmp::Ordering::{Greater, Less};
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let number = rand::thread_rng()
        .gen_range(1..=10);
    loop {
        println!("Please input number: ");
        let mut provided_number = String::new();

        io::stdin()
            .read_line(&mut provided_number)
            .expect("Read line failed");

        let guess: i32 = match provided_number.trim().parse() {
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
