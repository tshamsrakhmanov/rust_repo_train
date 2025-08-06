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
}
