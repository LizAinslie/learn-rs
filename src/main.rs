use std::io;

fn main() {
    println!("Hello, world! Type some text.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");

    println!("You said: {}", guess);
}
