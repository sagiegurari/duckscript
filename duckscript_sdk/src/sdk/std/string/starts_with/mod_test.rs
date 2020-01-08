use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = starts_with", "out");
}

#[test]
fn run_single_argument() {
    test::run_script_and_error(vec![create("")], "out = starts_with true", "out");
}

#[test]
fn run_two_arguments_equal() {
    test::run_script_and_validate(
        vec![create("")],
        "out = starts_with false false",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_two_arguments_not_starts_with() {
    test::run_script_and_validate(
        vec![create("")],
        "out = starts_with abcd bcd",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_two_arguments_starts_with() {
    test::run_script_and_validate(
        vec![create("")],
        "out = starts_with abcd abc",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
