use std::io;

#[derive(Debug)]
enum Coin {
    Penny,
    Dime,
    Any(u32)
}

fn main() {
    let mut input = String::new();

    println!("Enter your coin amount: ");

    io::stdin()
    .read_line(&mut input)
    .expect("Bad input");

    if let Ok(num) = input.trim().parse() {
        let coin = match num {
            1 => Coin::Penny,
            10 => Coin::Dime,
            any => Coin::Any(any)
        };

        println!("You got {:?}", coin);
    } else {
        println!("Well amount is a number, right...?");        
    }
}
