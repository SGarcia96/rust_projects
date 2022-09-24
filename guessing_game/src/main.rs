use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you guess. (number between 1-100)");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading");

    println!("You guessed: {guess}");
}
