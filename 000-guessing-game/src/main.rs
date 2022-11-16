use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number below 100!");
    let the_number = rand::thread_rng().gen_range(1..101);
    println!("The number is {}.", &the_number);

    loop {
        println!("What's your guess?");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // This re-declaration of guess in a different type is called *shadowing* 
        // let guess: u32 = guess.trim().parse().expect("Please type a numeric number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&the_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                print!("{}", "That's it!".green());
                break;
            }
        }
    }
}
