use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = greater_than", "out");
}

#[test]
fn run_single_arg() {
    test::run_script_and_error(vec![create("")], "out = greater_than 1", "out");
}

#[test]
fn run_not_numbers() {
    test::run_script_and_error(vec![create("")], "out = greater_than a b", "out");
}

#[test]
fn run_equal() {
    test::run_script_and_validate(
        vec![create("")],
        "out = greater_than 1.5 1.5",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_less() {
    test::run_script_and_validate(
        vec![create("")],
        "out = greater_than 1.5 2",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_greater() {
    test::run_script_and_validate(
        vec![create("")],
        "out = greater_than 1.5 1",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
