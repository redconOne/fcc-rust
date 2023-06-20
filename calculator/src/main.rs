use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first: String = args.nth(1).unwrap();
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result: f32 = 3.0;

    println!("{} {} {} = {}", first, operator, second, result)
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}
