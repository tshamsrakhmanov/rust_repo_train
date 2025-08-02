fn main() {
    let mut new_coin = Coin::Level1;
    let mut count = check_coin(&new_coin);
    println!("{count}");

    new_coin = Coin::Level2;
    count = check_coin(&new_coin);
    println!("{count}");

    new_coin = Coin::Level3;
    count = check_coin(&new_coin);
    println!("{count}");

    new_coin = Coin::Level4(CoinType::Used);
    count = check_coin(&new_coin);
    println!("{count}");

    new_coin = Coin::Level4(CoinType::Fresh);
    count = check_coin(&new_coin);
    println!("{count}");
}

enum Coin {
    Level1,
    Level2,
    Level3,
    Level4(CoinType),
}

enum CoinType {
    Fresh,
    Used,
}

fn check_coin(coin: &Coin) -> String {
    match coin {
        Coin::Level1 => String::from("Level 1 coin sends hello!"),
        Coin::Level2 => String::from("Level 2 coin sends hello!"),
        Coin::Level3 => String::from("Level 3 coin sends hello!"),
        Coin::Level4(CoinType::Used) => String::from("Level 4 USED coin sends hello!"),
        Coin::Level4(CoinType::Fresh) => String::from("Level 4 FRESH coin sends hello!"),
    }
}
