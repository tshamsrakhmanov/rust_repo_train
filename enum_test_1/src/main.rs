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

    new_coin = Coin::Level4;
    count = check_coin(&new_coin);
    println!("{count}");
}

enum Coin {
    Level1,
    Level2,
    Level3,
    Level4,
}

fn check_coin(coin: &Coin) -> u8 {
    match coin {
        Coin::Level1 => 1,
        Coin::Level2 => 2,
        Coin::Level3 => 3,
        Coin::Level4 => 4,
    }
}
