const VAR_CONST: i32 = 60 * 60 * 4;

fn main() {
    let mut q1 = 5;
    println!("Here is value of: {q1}");
    q1 = 6;
    println!("Here is mutated value of : {q1}");
    println!("Here is my constant: {VAR_CONST}");

    let var_bool: bool = false;
    println!("Here is bool type: {var_bool}");

    let char_a: char = 'a';
    let char_b = 'v';
    let char_c: char = '😻';
    println!("{char_a}\n{char_b}\n{char_c}");
}
