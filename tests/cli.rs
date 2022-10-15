use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// a test result type that can handle errors
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
/// test that the program dies when no args provided
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
       .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
/// test that the program runs
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
/// tests basic echo output
fn basic_echo() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
/// tests basic echo output without a newline
fn basic_echo_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
/// tests quotable output with a newline
fn basic_echo_quotes() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
/// tests quotable output without a newline
fn basic_echo_quotes_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

/// a helper function that runs a command and asserts that it contains the
/// expected output
fn run(args: &[&str], file_name: &str) -> TestResult {
    let expected = fs::read_to_string(file_name)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}