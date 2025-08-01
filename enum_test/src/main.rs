fn main() {
    let var1 = SomeEnum::Value1;
    let var2 = SomeEnum::Value2;
    println!("{var1:?}");
    println!("{var2:?}");
}

#[derive(Debug)]
enum SomeEnum {
    Value1,
    Value2,
}
