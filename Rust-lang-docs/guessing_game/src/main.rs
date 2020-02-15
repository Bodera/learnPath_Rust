use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    let secret = rand::thread_rng().gen_range(1, 200);

    println!("I can tell you that the secret generated is a number between 1 and 199");

    loop {
        println!("\nProvide your guess (integer numbers only):");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Guessed too small :("),
            Ordering::Greater => println!("Guessed too big xD"),
            Ordering::Equal => {
                println!("Congrats, you win :D");
                break;
            }
        }
    }
}
