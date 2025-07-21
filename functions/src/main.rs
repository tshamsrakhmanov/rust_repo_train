fn func_no_return(integer_value: i32, char_value: char) {
    println!("Incoming integer: {integer_value}, incoming char value: {char_value}");
}

fn func_with_return(input: i32) -> i32 {
    input + 5
}

fn main() {
    func_no_return(5, 'a');

    let value = func_with_return(14);
    println!("{value}");
}
