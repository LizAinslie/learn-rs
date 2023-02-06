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
        let arr = [1, 2, 3, 4, 5];

    }
}
