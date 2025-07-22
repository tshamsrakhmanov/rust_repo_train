fn main() {
    let x = 15;

    if x < 10 {
        println!("X is lower than 10");
    } else {
        println!("X is higher than 10");
    }

    println!("--------------");

    let condition = true;

    let z = if condition { 5 } else { 5 };

    println!("Value z: {z}");
    println!("--------------");
    let q = new_func(false);
    println!("{q}");
}

fn new_func(x: bool) -> i32 {
    if x { 5 } else { 10 }
}
