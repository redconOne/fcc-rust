use clap::{value_parser, Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    num1: f32,
    operator: char,
    num2: f32,
}

fn operate(operator: char, num1: f32, num2: f32) -> f32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '/' => num1 / num2,
        '*' | 'x' | 'X' => num1 * num2,
        _ => panic!("Invalid operator used."),
    }
}

fn output(num1: f32, operator: char, num2: f32, result: f32) -> String {
    format!("{} {} {} = {}", num1, operator, num2, result)
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("Rust Calc")
        .version("0.1.0")
        .author("Ming Lee Ng <mingleeng@gmail.com>")
        .about("A simple arithmetic calculator written in Rust")
        .arg(
            Arg::new("first")
                .value_name("FIRST_NUMBER")
                .help("First operand")
                .required(true)
                .value_parser(value_parser!(f32)),
        )
        .arg(
            Arg::new("operator")
                .value_name("OPERATOR")
                .help("Arithmetic operator")
                .required(true),
        )
        .arg(
            Arg::new("second")
                .value_name("SECOND_NUMBER")
                .help("Second operand")
                .required(true)
                .value_parser(value_parser!(f32)),
        )
        .get_matches();

    let first: f32 = *matches.get_one("first").unwrap();
    let second: f32 = *matches.get_one("second").unwrap();
    let operator_str: &String = matches.get_one::<String>("operator").unwrap();
    let operator: char = operator_str.chars().next().unwrap();

    Ok(Config {
        num1: first,
        operator,
        num2: second,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let total: f32 = operate(config.operator, config.num1, config.num2);
    let result: String = output(config.num1, config.operator, config.num2, total);
    println!("{}", result);
    Ok(())
}
