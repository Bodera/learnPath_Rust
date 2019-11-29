use std::io;

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    println!("Provide your guess (integer numbers only):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);
}