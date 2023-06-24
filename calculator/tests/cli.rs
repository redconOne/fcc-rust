use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "calculator";

//          Failure Tests
// --------------------------------------------
#[test]
fn dies_with_one_arg() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .env("exit", "1")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn dies_with_two_args() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .arg("-")
        .env("exit", "1")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn dies_with_improper_args() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .arg("2")
        .arg("3")
        .env("exit", "1")
        .assert()
        .failure();
    Command::cargo_bin(PRG)?
        .arg("a")
        .arg("+")
        .arg("2")
        .env("exit", "1")
        .assert()
        .failure();
    Ok(())
}

//          Addition Tests
//----------------------------------------
#[test]
fn integer_addition_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .arg("+")
        .arg("7")
        .assert()
        .stdout(predicate::eq("1 + 7 = 8\n"));

    Ok(())
}

#[test]
fn decimal_addition_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(".9993")
        .arg("+")
        .arg(".0007")
        .assert()
        .stdout(predicate::eq("0.9993 + 0.0007 = 1\n"));
    Ok(())
}

//          Subtraction Tests
//----------------------------------------

#[test]
fn integer_subtraction_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .arg("-")
        .arg("7")
        .assert()
        .stdout(predicate::eq("1 - 7 = -6\n"));

    Ok(())
}

#[test]
fn decimal_subtraction_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(".9993")
        .arg("-")
        .arg(".0007")
        .assert()
        .stdout(predicate::eq("0.9993 - 0.0007 = 0.9986\n"));
    Ok(())
}

//              Multiplication Tests
//------------------------------------------------------------

#[test]
fn integer_multiplication_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .arg("*")
        .arg("7")
        .assert()
        .stdout(predicate::eq("1 * 7 = 7\n"));

    Command::cargo_bin(PRG)?
        .arg("123")
        .arg("x")
        .arg("0")
        .assert()
        .stdout(predicate::eq("123 x 0 = 0\n"));

    Ok(())
}

#[test]
fn decimal_multiplication_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(".9993")
        .arg("X")
        .arg(".0007")
        .assert()
        .stdout(predicate::eq("0.9993 X 0.0007 = 0.00069951\n"));
    Ok(())
}

//                  Division Tests
//-----------------------------------------------------------
#[test]
fn integer_division_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("1")
        .arg("/")
        .arg("7")
        .assert()
        .stdout(predicate::eq("1 / 7 = 0.14285715\n"));
    Command::cargo_bin(PRG)?
        .arg("12")
        .arg("/")
        .arg("3")
        .assert()
        .stdout(predicate::eq("12 / 3 = 4\n"));
    Ok(())
}

#[test]
fn decimal_division_test() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(".9993")
        .arg("/")
        .arg(".0007")
        .assert()
        .stdout(predicate::eq("0.9993 / 0.0007 = 1427.5715\n"));
    Command::cargo_bin(PRG)?
        .arg("3")
        .arg("/")
        .arg("4")
        .assert()
        .stdout(predicate::eq("3 / 4 = 0.75\n"));
    Ok(())
}
