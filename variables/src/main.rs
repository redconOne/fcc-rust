fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);

    const UNCHANGEABLE_VALUE: u32 = 100_000;
    println!("Our constant value is: {}", UNCHANGEABLE_VALUE);

    // Scaler data types = Integers, Floating points, Booleans, Characters

    // Integers

    let a: i32 = 98_222; //Decimal
    let b: i32 = 0xff; //Hex
    let c: i32 = 0o77; //Octal
    let d: i32 = 0b1111_0000; //Binary
    let e: u8 = b'A'; // Byte (u8 only)
    let f: u8 = 255;
    let g: i8 = 127;
    println!("{}{}{}{}{}{}{}", a, b, c, d, e, f, g);

    // Floating Points
    let f: f64 = 2.0;
    let g: f32 = 3.0;
    println!("{}{}", f, g);

    // Booleans
    let h: bool = true;
    let i: bool = false;
    println!("{}{}", h, i);

    // Character
    let j: char = 'j';
    println!("{}", j);

    // Compound types
    // Tuples
    let tup = ("Ming's tuple", 100_000);
    let (thing1, thing2) = tup;
    println!("{}{}", thing1, thing2);

    // Arrays - Fixed length
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let eight_zeroes = [0; 8];
    println!("{:?}{}{:?}", error_codes, not_found, eight_zeroes);

    my_function();
    other_function(35);

    let mut counter = 0;
    loop {
        if counter >= 10 {
            break;
        }
        counter += 1;
        println!("{}", counter);
    }

    let a = [10, 20, 30, 40, 50];
    for item in a.iter() {
        println!("{}", item);
    }

    for num in 1..=5 {
        println!("{}", num);
    }
}

fn my_function() {
    println!("Another function.");
}

fn other_function(x: i32) {
    println!("I am {} year(s) old.", x);
}
