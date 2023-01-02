use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number (you can't), give input now:");
    println!("If you want to quit anytime, type 'quit'");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("You can't even input a line...");

        let guess = guess.trim();

        if guess == "quit" {
            println!("Quitting...");
            break;
        }
    
        let guess : i32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("What a shame... atleast input a proper number");
                continue;
            }
        };
    
        println!("Your guess was {guess}");
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Nah, you don't deserve to win, it's too low..."),
            Ordering::Greater => println!("Woah slow down... it's too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }

    println!("The secret number was: {secret_num}");
}
