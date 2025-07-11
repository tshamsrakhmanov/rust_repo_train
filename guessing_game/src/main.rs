use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("need to input valid value!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match &guess.cmp(&_secret_number) {
            Ordering::Equal => {
                println!("Equal");
                break;
            }
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
        }
    }
}
