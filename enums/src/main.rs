#[derive(Debug)]
enum UsState {
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let value1 = value_in_cents(coin1);
    println!("Coin 1: {}", value1);

    let coin2 = Coin::Nickel;
    let value2 = value_in_cents(coin2);
    println!("Coin 2: {}", value2);

    let coin3 = Coin::Dime;
    let value3 = value_in_cents(coin3);
    println!("Coin 3: {}", value3);

    let coin4 = Coin::Quarter(UsState::Texas);
    let value4 = value_in_cents(coin4);
    println!("Coin 4: {}", value4);

    let first = Some(7_i32);
    let second = plus_one(first);
    println!("{}", second.unwrap());
}
