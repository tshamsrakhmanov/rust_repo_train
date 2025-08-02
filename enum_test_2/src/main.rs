use core::option::Option;

fn main() {
    let var1 = Some(5);
    let var2 = add_value(var1);
    println!("{var2:?}");
    let var3 = add_value(None);
    println!("{var3:?}");

    let var5 = TypeTest::CriticalTest;
    let var6 = TypeTest::CriticalTest;
    let var7 = TypeTest::CriticalTest;
    let var8 = TypeTest::CriticalTest;

    match var5 {
        TypeTest::CriticalTest => {
            println!("CriticalTest")
        }
        _ => println!("skip this stage"),
    }

    match var6 {
        TypeTest::ValidTest => println!("!"),
        _ => println!("?"),
    }
}

fn add_value(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 5),
    }
}

enum TypeTest {
    ValidTest,
    NonValidTest,
    CriticalTest,
    SkippableTest,
}
