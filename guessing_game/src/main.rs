use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Enter your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("need to input valid value!");

    print!("Your guess: {guess}");

    match _secret_number.cmp(guess) {
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Greater"),
        Ordering::Less => println!("Less"),
    }
}
