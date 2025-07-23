fn main() {
    let str1 = return_string();
    println!("{str1}");

    let str2 = String::from("String for printing inside function");
    print_str_by_referal(&str2);

    let str3: String = String::from("String to modify and print");
    append_no_modify_print(&str3);
}

fn return_string() -> String {
    let z: String = String::from("... return string from funciton");
    z
}

fn print_str_by_referal(str: &String) {
    println!("{str}");
}

fn append_no_modify_print(str: &String) {
    let mut temp_str: String = String::new();
    temp_str.push_str(str);
    temp_str.push_str("[THIS SECTION FROM FUNCTION]");
    println!("{temp_str}");
}
