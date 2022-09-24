use std::io;
use rand::Rng;

fn main() {
    println!("*** Guess the number! ***");
    println!("Please input you guess. (number between 1-100)");

    let secret_number = rand::thread_rng().gen_range(0..100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading the line");

    println!("You guessed: {guess}");
    println!("The secret number is: {secret_number}");
}
