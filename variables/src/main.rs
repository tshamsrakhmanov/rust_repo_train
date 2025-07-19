const VAR_CONST: i32 = 60 * 60 * 4;

fn main() {
    // integer example
    let mut q1 = 5;
    println!("Here is value of: {q1}");
    q1 = 6;
    println!("Here is mutated value of : {q1}");
    println!("Here is my constant: {VAR_CONST}");

    // bool
    let var_bool: bool = false;
    println!("Here is bool type: {var_bool}");

    // char-s
    let char_a: char = 'a';
    let char_b = 'v';
    let char_c: char = '😻';
    println!("{char_a}\n{char_b}\n{char_c}");

    // tuples
    let new_tup: (i32, char, u8) = (1234, 'a', 4);
    let (x, y, z) = new_tup;
    println!("Here is unpacking tuple by creating new var - destructive method: {x} {y} {z}");

    let new_var = new_tup.0;
    let new_var1 = new_tup.1;
    let new_var2 = new_tup.2;
    println!(
        "Here is unpacking of tuple by reference with '.' declaration: {new_var} {new_var1} {new_var2}"
    );

    //arrays
    let array_1 = [3; 5];
    let array_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let array_3: [i32; 5];

    let first_entry = array_1[0];
    println!("Fisrt entry of array_1: {first_entry}");
}
