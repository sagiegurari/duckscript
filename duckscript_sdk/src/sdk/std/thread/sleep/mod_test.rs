use super::*;
use crate::test;
use crate::test::CommandValidation;
use std::time;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create("")],
        "out = sleep",
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}

#[test]
fn run_0() {
    test::run_script_and_validate(
        vec![create("")],
        "out = sleep 0",
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}

#[test]
fn run_negative_value() {
    test::run_script_and_error(vec![create("")], "out = sleep -1", "out");
}

#[test]
fn run_positive_value() {
    let now = time::Instant::now();

    test::run_script_and_validate(
        vec![create("")],
        "out = sleep 10",
        CommandValidation::Match("out".to_string(), "10".to_string()),
    );

    assert!(now.elapsed() >= Duration::from_millis(10));
}
