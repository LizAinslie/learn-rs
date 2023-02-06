use std::io::{self, Write};
mod funcs;

fn main() {
    // 3.1 - Variables & Mutability
    { // mutability
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    { // constants
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    }
    
    { // shadowing
        let x = 5;
        let x = x + 1;

        {
            let x = x * 2;
            println!("Value of x (inner): {x}"); // should be 12
        }

        println!("Value of x (outer): {x}"); // should be 6, since the multiplication was shadowed
    }

    // 3.2 - Data Types
    { // number types
        let some_binary = 0b0000_1111;
        let some_hex = 0xff;
        let some_octal = 0o777;
        let some_float = 2.0; // f64 - default floating point type
        let some_other_float: f32 = 3.0; // f32
    }

    { // arithmetic
        let sum = 5 + 10; // addition
        let difference = 95.5 - 4.3; // subtraction
        let product = 4 * 30; // multiplication

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        let remainder = 43 % 5; // remainder (modulo)
    }

    { // booleans
        let t = true;
        let f: bool = false;
    }

    { // characters
        let some_char = 'c';
        let some_unicode_char = '😻';
    }

    { // tuples
        let some_tuple: (i32, f64, u8) = (200, 3.1, 1);
        let (x, y, z) = some_tuple;
        println!("{x}, {y}, {z}");

        let two_hundred = some_tuple.0;
        let three_point_one = some_tuple.1;
        let one = some_tuple.2;
    }

    { // arrays
        let some_arr = [1, 2, 3, 4, 5];
        let some_other_arr: [i32; 5] = [1, 2, 3, 4, 5]; // array types: [type, length]
        let five_threes = [3; 5]; // an array of five 3's. [value; length]
        
        // array index access
        let first = some_arr[0];
        let second = some_arr[1];
    }

    { // ask for an array index, this code should panic if index is oob
        let a = [1, 2, 3, 4, 5];
        print!("Please enter an array index. >4 should panic: ");
        io::stdout().flush().unwrap();

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }

    funcs::entry(); // functions

    // 3.5 - Control Flow
    { // if / else
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    { // else-if statements
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    { // conditional in assignment
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is {number}");
    }

    { // loop labels
        let mut count = 0;
        'counting_up: loop { // name this loop counting_up <-------------\
            println!("count = {count}"); //                              |
            let mut remaining = 10; //                                   |
            //                                                           |
            loop { // <------------------------------------\             |
                println!("remaining = {remaining}"); //    |             |
                if remaining == 9 { //                     |             |
                    break; // break the inner-most loop ---/             |
                } //                                                     |
                if count == 2 { //                                       |
                    break 'counting_up; // break the counting_up loop ---/
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    { // fibonacci example
        fn fib(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }

            fib(n - 1) + fib(n - 2)
        }

        println!("Fib(5) = {}", fib(5));
        println!("Fib(12) = {}", fib(12));
    }
}
