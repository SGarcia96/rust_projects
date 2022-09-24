use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("*** Guess the number! ***");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input you guess. (number between 1-100)");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading the line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        }; 
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct! You Win!");
                break;
            }
        }
    }
}
