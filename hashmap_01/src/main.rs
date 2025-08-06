use std::collections::HashMap;

fn main() {
    // make a hash map
    let mut hashmap1: HashMap<String, i32> = HashMap::new();

    //populate it with cycle
    for i in 0..6 {
        let cast_to_str = i.to_string();
        let mut temp = String::from("key");
        temp.push_str(&cast_to_str);
        hashmap1.insert(temp, i);
    }
    println!("{hashmap1:?}");

    // print by getting value using key getter
    for (k, _v) in &hashmap1 {
        let val = hashmap1[k];
        println!("{val}");
    }

    // check, if a key is Occupied or Vacant and return back a value
    match hashmap1.entry(String::from("key2")) {
        std::collections::hash_map::Entry::Occupied(some_value) => println!("{some_value:?}"),
        std::collections::hash_map::Entry::Vacant(some_value) => println!("{some_value:?}"),
    }

    println!("----------------------------------------");

    // try of change hash map in runtime  - but failed.....
    let mut hashmap2: HashMap<String, u8> = HashMap::new();

    let str = "some random string with some random values";

    for word in str.split(" ") {
        let str1 = String::from(word);
        hashmap2.entry(str1).or_default();
    }

    println!("{hashmap2:?}");

    println!("----------------------");
    // example of changing hash map in runtime according to RustBook
    // somehow works, black magic
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    println!("-------------------------------------");

    let mut hashmap3: HashMap<String, u8> = HashMap::new();

    let str1 = "some string some string some string";
    println!("{str1}");

    for word in str1.split_whitespace() {
        println!("{word}");
        if hashmap3.entry(word) {
            // ..
        }
    }
}
