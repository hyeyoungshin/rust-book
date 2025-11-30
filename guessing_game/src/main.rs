// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// Programmning a Guessing Game

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // `let mut` allows programmer to reassign the variable `guess`

        io::stdin()
            .read_line(&mut guess) // `&mut` giallows the function to modify what `guess` points to
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // go to the next iteration
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
