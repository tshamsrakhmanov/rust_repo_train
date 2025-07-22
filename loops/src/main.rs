fn main() {
    example_2();
    println!("------------------------------------------------");
    example_3();
    println!("--------------------------------------------");
    example_4();
    println!("--------------------------------------------");
    example_5();
    println!("--------------------------------------------");
    example_6();
    println!("--------------------------------------------");
    example_7();
}

fn example_2() {
    let mut counter = 0;
    let x = loop {
        counter += 1;

        if counter == 140 {
            break counter;
        }
    };
    println!("{x}");
}

fn example_3() {
    let mut x = 0;

    'loop1: loop {
        let mut y = 0;
        x += 1;
        'loop2: loop {
            println!("{x}{y}");
            if y == 5 {
                break 'loop2;
            }
            if x == 5 {
                break 'loop1;
            }
            y += 1;
        }
    }
}

fn example_4() {
    let mut x = 10;

    while x != -1 {
        println!("...{x}");
        x -= 1;
    }
    println!("Poehali!");
}

fn example_5() {
    let array_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let mut x = 9;

    while x != 0 {
        println!("value is:{}", array_1[x]);
        x -= 1;
    }
}

fn example_6() {
    let a = [1, 2, 3, 4, 5];

    for i in a {
        println!("{}", i);
    }
}

fn example_7() {
    for i in 150..157 {
        println!("{}", i);
    }
}
