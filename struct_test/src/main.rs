fn main() {
    let t1 = ImmutableStructure {
        value1: 4,
        value2: 3,
    };

    let mut new_user = User {
        name: String::from("asdfasfd"),
        surname: String::from("qwerqwer"),
        active: true,
        log_in_count: 5,
    };

    new_user.surname = String::from("qweqwer");
    new_user.active = false;
    new_user.name = String::from("zcxvzxcv");
    new_user.log_in_count = 5;

    let value1 = t1.value1;
    let value2 = t1.value2;
    println!("{value1} {value2}");

    println!("{t1:?}");
    println!("{new_user:?}");
}

#[derive(Debug)]
struct User {
    name: String,
    log_in_count: i32,
    surname: String,
    active: bool,
}

#[derive(Debug)]
struct ImmutableStructure {
    value1: i8,
    value2: i8,
}
