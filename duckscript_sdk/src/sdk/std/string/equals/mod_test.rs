use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_fail(vec![create("")], "out = eq");
}

#[test]
fn run_single_argument() {
    test::run_script_and_fail(vec![create("")], "out = eq true");
}

#[test]
fn run_two_arguments_equal() {
    test::run_script_and_validate(
        vec![create("")],
        "out = eq false false",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_two_arguments_not_equal() {
    test::run_script_and_validate(
        vec![create("")],
        "out = eq 1 false",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
