extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let low = 1;
    let high = 101;
    let secret_number = rand::thread_rng().gen_range(low, high);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
             Ordering::Less => println!("Too small!!"),
             Ordering::Greater => println!("Too big"),
             Ordering::Equal => {
                 println!("You win!");
                 break;
             }
        }
    }
}
