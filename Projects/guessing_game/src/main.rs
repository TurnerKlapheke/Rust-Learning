extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    let mut max = 100;
    let mut min = 1;
    
    loop {
        println!("Please input your guess, the number is between {} and {}", min, max);
        
        let mut guess = String::new(); //mutable
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // guessing
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                if guess > min {
                    min = guess + 1;
                }
            },
            Ordering::Greater => {
                println!("Too big!");
                if guess < max {
                    max = guess - 1;
                }
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}