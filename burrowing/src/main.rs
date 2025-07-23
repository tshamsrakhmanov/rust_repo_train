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
    modify_str_by_add_prefif(&mut str5);
    modify_str_by_add_prefif(&mut str5);
    println!("{str5}");

    let mut str6: String = String::from("[base string]");
    let str7: String = String::from("[add by function]");
    append_str_to_str(&mut str6, &str7);
    println!("{str6}");
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

fn modify_str_by_add_prefif(str: &mut String) {
    str.push_str("[add by fucntion to passed variable]");
}

fn append_str_to_str(str1: &mut String, str2: &String) {
    str1.push_str(str2);
}
