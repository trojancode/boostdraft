use std::process::{Command, Output};

fn run_checker(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_checker"))
        .args(arguments)
        .output()
        .expect("checker should run")
}

fn assert_output(output: Output, expected_stdout: &str) {
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_stdout);
    assert!(output.stderr.is_empty());
}

#[test]
fn prints_valid_for_a_balanced_input() {
    assert_output(run_checker(&["<a><b>text</b></a>"]), "Valid\n");
}

#[test]
fn prints_invalid_for_an_unbalanced_input() {
    assert_output(run_checker(&["<a><b></a></b>"]), "Invalid\n");
}

#[test]
fn prints_invalid_when_the_first_argument_is_missing() {
    assert_output(run_checker(&[]), "Invalid\n");
}
