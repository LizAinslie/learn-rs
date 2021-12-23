use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the number guessing game.");
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("Shh! Secret number is {}", secret_num);


    loop {
        eprint!("Guess a number: ");

        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str).expect("Failed to read input.");
        let guess_result = guess_str.trim().parse();

        if let Err(_) = guess_result {
            println!("Input must be a number.");
            continue;
        }

        let guess_num: u32 = guess_result.unwrap();
        println!("You guessed {}.", guess_num);
        match guess_num.cmp(&secret_num) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
