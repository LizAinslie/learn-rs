fn print_x(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn entry() {
    print_x(5);
    print_labeled_measurement(5, 'h');

    { // expression blocks
        let y = {
            let x = 3;
            x + 1
        };

        print_x(y);
    }

    { // return values
        let x = five();
        print_x(x);

        let y = plus_one(5);
        print_x(y);
    }
}
