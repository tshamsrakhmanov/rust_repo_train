fn main() {
    let str1 = String::from("some words hello");
    let str_len = get_len(&str1);
    println!("{str_len}");
    let str_to_bytes = str1.as_bytes();
    for (x, &y) in str_to_bytes.iter().enumerate() {
        if y == b' ' {
            println!("found space");
        } else if y == b's' {
            println!("found S");
        }
        println!("{x} {y}");
    }
    println!("---------------------------------------");

    let some_str = String::from("hello world");

    let word1 = &some_str[..5];
    let word2 = &some_str[6..];

    println!("{word1}");
    println!("{word2}");
}

fn get_len(input_str: &String) -> usize {
    let length = input_str.len();
    length
}
