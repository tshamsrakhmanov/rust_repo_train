fn main() {
    let mut new_coin = Coin::Base;
    let mut count = check_coin(&new_coin);
    println!("{count}");
    new_coin = Coin::UpperBase;
    count = check_coin(&new_coin);
    println!("{count}");
    new_coin = Coin::NewBase;
    count = check_coin(&new_coin);
    println!("{count}");
    new_coin = Coin::HighBase;
    count = check_coin(&new_coin);
    println!("{count}");
}

enum Coin {
    Base,
    UpperBase,
    NewBase,
    HighBase,
}

fn check_coin(coin: &Coin) -> u8 {
    match coin {
        Coin::Base => 1,
        Coin::UpperBase => 2,
        Coin::NewBase => 3,
        Coin::HighBase => 4,
    }
}
