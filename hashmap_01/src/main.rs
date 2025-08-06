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
}
