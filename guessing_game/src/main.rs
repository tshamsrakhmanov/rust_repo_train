use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number_guessed_by_machine = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");
        let mut guess_by_user = String::new();

        io::stdin()
            .read_line(&mut guess_by_user)
            .expect("need to input valid value!");

        let guess_parsed: i32 = match guess_by_user.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess_parsed}");

        match &guess_parsed.cmp(&secret_number_guessed_by_machine) {
            Ordering::Equal => {
                println!("Equal");
                break;
            }
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
        }
    }
}
