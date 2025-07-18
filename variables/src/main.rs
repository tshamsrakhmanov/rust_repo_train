const MY_SIMPLE_VARIABLE_CONSTANT: i32 = 60 * 60 * 4;

fn main() {
    let mut q1 = 5;
    println!("Here is value of: {q1}");
    q1 = 6;
    println!("Here is mutated value of : {q1}");
    println!("Here is my constant: {MY_SIMPLE_VARIABLE_CONSTANT}");
}
