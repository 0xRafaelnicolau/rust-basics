fn main() {
    is_less_than_ten(5);
    is_divisible_by_two(6);
    from_loop();
}

fn is_less_than_ten(x: u8) {
    if x < 10 {
        println!("{x} is smaller than 10");
    } else {
        println!("{x} is bigger than 10");
    }
}

fn is_divisible_by_two(x: u32) {
    if x % 2 == 0 {
        println!("{x} is divisible by 2!");
    } else {
        println!("{x} is not divisble by 2!");
    }
}

fn from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}
