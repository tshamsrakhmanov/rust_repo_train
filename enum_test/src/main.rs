fn main() {
    let var1 = SomeEnum::Value1;
    let var2 = SomeEnum::Value2;
    println!("{var1:?}");
    println!("{var2:?}");
    let var3 = MovementType::Running(14);
    let var4 = MovementType::Trolling(12);
    let var5 = MovementType::Walking(20);
    println!("{var3:?}");
    println!("{var4:?}");
    println!("{var5:?}");
    let address_v4 = IpAddress::V4(String::from("12.12.12.12"));
}

#[derive(Debug)]
enum SomeEnum {
    Value1,
    Value2,
}

#[derive(Debug)]
enum MovementType {
    Walking(u8),
    Trolling(u8),
    Running(u8),
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
}
