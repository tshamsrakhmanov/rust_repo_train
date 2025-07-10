use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Computers guess:{_secret_number}");
    println!("Enter your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("need to input valid value!");

    let guess: i32 = guess
        .trim()
        .parse()
        .expect("Please enter a valid number!!!");

    println!("Your guess: {guess}");

    println!();

    match &guess.cmp(&_secret_number) {
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Greater"),
        Ordering::Less => println!("Less"),
    }
}
