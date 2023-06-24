use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "calculator";

fn passing(args: &[&str], expected: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(predicate::eq(expected));

    Ok(())
}

fn failing(args: &[&str]) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .env("exit", "1")
        .assert()
        .failure();
    Ok(())
}

//          Failure Tests
// --------------------------------------------
#[test]
fn dies_with_one_arg() -> TestResult {
    failing(&["1"])
}

#[test]
fn dies_with_two_args() -> TestResult {
    failing(&["1", "-"])
}

#[test]
fn dies_with_improper_operator() -> TestResult {
    failing(&["1", "2", "3"])
}

#[test]
fn dies_with_improper_operand() -> TestResult {
    failing(&["a", "+", "2"])
}

#[test]
fn test_failing() -> TestResult {
    failing(&["1", "+", "2"])
}

//          Addition Tests
//----------------------------------------
#[test]
fn integer_addition_test() -> TestResult {
    passing(&["1", "+", "7"], "1 + 7 = 8\n")
}

#[test]
fn decimal_addition_test() -> TestResult {
    passing(&[".9993", "+", ".0007"], "0.9993 + 0.0007 = 1\n")
}

//          Subtraction Tests
//----------------------------------------

#[test]
fn integer_subtraction_test() -> TestResult {
    passing(&["1", "-", "7"], "1 - 7 = -6\n")
}

#[test]
fn decimal_subtraction_test() -> TestResult {
    passing(&[".9993", "-", ".0007"], "0.9993 - 0.0007 = 0.9986\n")
}

//              Multiplication Tests
//------------------------------------------------------------

#[test]
fn integer_multiplication_test() -> TestResult {
    passing(&["6", "*", "7"], "6 * 7 = 42\n")
}

#[test]
fn multiplication_with_zero_test() -> TestResult {
    passing(&["0", "x", "123"], "0 x 123 = 0\n")
}

#[test]
fn decimal_multiplication_test() -> TestResult {
    passing(&["0.9993", "X", ".0007"], "0.9993 X 0.0007 = 0.00069951\n")
}

//                  Division Tests
//-----------------------------------------------------------
#[test]
fn int_to_int_division_test() -> TestResult {
    passing(&["12", "/", "3"], "12 / 3 = 4\n")
}

#[test]
fn int_into_float_division_test() -> TestResult {
    passing(&["1", "/", "7"], "1 / 7 = 0.14285715\n")
}

#[test]
fn basic_division_test() -> TestResult {
    passing(&["3", "/", "4"], "3 / 4 = 0.75\n")
}

#[test]
fn advanced_division_test() -> TestResult {
    passing(&[".9993", "/", ".0007"], "0.9993 / 0.0007 = 1427.5715\n")
}
