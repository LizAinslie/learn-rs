use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number! (1-100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Psst! The secret number is {secret_number}");

    loop {
        print!("Input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number.");
                continue;
            },
        };

        print!("You guessed {}, ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }
    }
}

