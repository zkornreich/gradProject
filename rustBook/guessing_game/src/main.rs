use std::io;
use std::cmp::Ordering;
use rand::{rng, Rng};

fn main() {
    let sec_num: u32 = rng().random_range(1..=100);
    loop {
        println!("Guess the number! Submit a guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // println!("The Secret Number was: {sec_num}");
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; }
        }
    }
}