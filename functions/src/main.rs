fn new_function(integer_value: i32, char_value: char) {
    println!("Incoming integer: {integer_value}, incoming char value: {char_value}");
}

fn new_function_with_return(input: i32) -> i32 {
    input + 5
}

fn main() {
    new_function(5, 'a');

    let value = new_function_with_return(14);
    println!("{value}");
}
