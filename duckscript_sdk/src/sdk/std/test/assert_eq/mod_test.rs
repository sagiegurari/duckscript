use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_fail(vec![create("")], "out = assert_eq");
}

#[test]
fn run_single_argument() {
    test::run_script_and_fail(vec![create("")], "out = assert_eq true");
}

#[test]
fn run_two_arguments_equal() {
    test::run_script_and_validate(
        vec![create("")],
        "out = assert_eq false false",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_two_arguments_equal_with_error_message() {
    test::run_script_and_validate(
        vec![create("")],
        "out = assert_eq false false message",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_two_arguments_not_equal() {
    test::run_script_and_fail(vec![create("")], "out = assert_eq 1 false");
}

#[test]
fn run_two_arguments_not_equal_error_message() {
    test::run_script_and_fail(vec![create("")], r#"out = assert_eq 1 false "test error""#);
}
