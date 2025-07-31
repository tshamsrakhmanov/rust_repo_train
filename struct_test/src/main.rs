fn main() {
    let mut new_user = User {
        name: String::from("asdfasfd"),
        surname: String::from("qwerqwer"),
        active: true,
        log_in_count: 5,
    };

    new_user.surname = String::from("qweqwer");

    // println!("{new_user}");
}

struct User {
    name: String,
    log_in_count: i32,
    surname: String,
    active: bool,
}
