fn main() {
    let str1 = return_string();
    println!("{str1}");

    let str2 = String::from("String for printing inside function");
    print_str_by_referal(&str2);

    let str3: String = String::from("String to modify and print");
    append_no_modify_print(&str3);

    let str4: String = String::from("WORD");
    let str5 = add_prefix_suffix(&str4);
    println!("{str5}");

    let mut str5: String = String::new();
    str5.push_str("[Base value of variable]");
    modifu_add_prefix(&mut str5);
    println!("{str5}");
}

fn return_string() -> String {
    let z: String = String::from("... return string from funciton");
    z
}

fn print_str_by_referal(str: &String) {
    println!("{str}");
}

fn append_no_modify_print(str: &String) {
    let mut i: String = String::new();
    i.push_str(str);
    i.push_str("[THIS SECTION FROM FUNCTION]");
    println!("{i}");
}

fn add_prefix_suffix(str: &String) -> String {
    let mut i: String = String::new();
    i.push_str("[PREFIX]");
    i.push_str(str);
    i.push_str("[SUFFIX]");
    i
}

fn modifu_add_prefix(str: &mut String) {
    str.push_str("[add by fucntion to passed variable]");
}
