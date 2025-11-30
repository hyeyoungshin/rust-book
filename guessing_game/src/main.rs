// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// Programmning a Guessing Game

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // `let mut` allows programmer to reassign 
                                   // the variable `guess`

    io::stdin()
        .read_line(&mut guess) // `&mut` giallows the function to modify what 
                               // `guess` points to
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}
