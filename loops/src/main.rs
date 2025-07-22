fn main() {
    example_2();
    println!("------------------------------------------------");
    example_3();
    println!("--------------------------------------------");
    example_4();
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
