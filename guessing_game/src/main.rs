use std::io;

fn main() {
    println!("Enter your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("need to input valid value!");
    print!("Your guess: {guess}",)
}
