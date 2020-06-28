use std::io; //brings to scope io module from std library
use std::cmp::Ordering; //brings to scope Compare method plus Ordering type from std library
use rand::Rng; //brings to scope Rng trait from rand crate

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    let secret = rand::thread_rng().gen_range(1, 200); //initialize a immutable variable that can store integers from 1 to 199

    println!("I can tell you that the secret generated is a number between 1 and 199");

    loop { //starts a loop that breaks when user input a number that matches with value stored on variable secret
        println!("\nProvide your guess (integer numbers only): ");

        let mut guess = String::new();  //initialize a mutable variable that can store strings

        io::stdin().read_line(&mut guess) //handling user input passing the mutable reference of guess as an argument
            .expect("Failed to read line."); //in case of 'Err' enum variation

        let guess: u32 = match guess.trim().parse() { //reusing guess variable name using shadowing technique, also converts the inputed string into a unsigned int of 32-bit
            Ok(num) => num, //in case of 'Ok' enum variation, guess will store the value of num
            Err(_) => continue, //in case of 'Err' enum variation, ignores condition violation and asks for a new guess
        };

        println!("Your guess was: {}", guess); //string interpolation

        match guess.cmp(&secret) { //initiates a match expression comparing user guess with generated secret
            Ordering::Less => println!("Guessed too small :("), //check if the variant of Ordering returned less
            Ordering::Greater => println!("Guessed too big xD"), //check if the variant of Ordering returned greater
            Ordering::Equal => { //check if the variant of Ordering returned equal, then quit the loop
                println!("Congrats, you win :D");
                break;
            }
        }
    }
}
