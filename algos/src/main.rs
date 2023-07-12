use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let rand: u32 = generate_rand();
    println!("Random number: {}", rand);

    let rand_in_range: u32 = generate_rand_in_range(1, 11);
    println!("Random number between 1-11: {}", rand_in_range);

    let dice_roll = roll_dice();
    println!("Dice roll: {}", dice_roll);

    let alpha_password = generate_alphanumeric_password();
    println!("Alphanumeric password: {}", alpha_password);

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let user_defined_password = generate_user_defined_password(CHARSET, 5);
    println!("User defined password {}", user_defined_password);
}

/// Generates a random u32 number
fn generate_rand() -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen::<u32>()
}

/// Generates a random u32 between two bounds, lower inclusive, upper exclusive
fn generate_rand_in_range(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(lower_bound..upper_bound)
}

/// Generate a number 1-6 simulating a dice-roll
fn roll_dice() -> u8 {
    let mut rng = rand::thread_rng();

    rng.gen_range(1..7)
}

/// Generate a password for ASCII characters A-Z, a-z, and 0-9
fn generate_alphanumeric_password() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

/// Generate a password for user-defined characters
fn generate_user_defined_password(charset: &[u8], length: u8) -> String {
    let mut rng = thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}
