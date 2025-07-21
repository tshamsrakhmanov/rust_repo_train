const VAR_CONST: i32 = 60 * 60 * 4;

fn main() {
    // integer example
    println!("INTEGERS");
    let mut q1 = 5;
    println!("Here is value of: {q1}");
    q1 = 6;
    println!("Here is mutated value of : {q1}");
    println!("Here is my constant: {VAR_CONST}");
    println!("---------------------------------------");
    println!("BOOL");

    // bool
    let var_bool: bool = false;
    println!("Here is bool type: {var_bool}");
    println!("---------------------------------------");
    println!("CHARS");

    // char-s
    let char_a: char = 'a';
    let char_b = 'v';
    let char_c: char = '😻';
    println!("{char_a}\n{char_b}\n{char_c}");
    println!("---------------------------------------");
    println!("TUPLES");

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
    println!("---------------------------------------");
    println!("ARRAYS");

    //arrays
    let array_1 = [3; 5];
    let array_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let array_3: [i32; 5] = [1234, 2345, 3456, 4567, 5678];
    let array_strs = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_entry = array_1[0];
    let second_entry = array_2[1];
    let third_entry = array_strs[4];
    let one_more_entry = array_3[0];
    println!("Fisrt entry of array_1: {first_entry}");
    println!("Second entry of array_2: {second_entry}");
    println!("Third entry of array_strs: {third_entry}");
    println!("One more entry: {one_more_entry}");
}
