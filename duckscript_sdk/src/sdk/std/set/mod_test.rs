use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_validate(vec![create("")], "out = set", CommandValidation::None);
}

#[test]
fn run_single_argument() {
    test::run_script_and_validate(
        vec![create("")],
        "out = set test",
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}

#[test]
fn run_multiple_arguments() {
    test::run_script_and_validate(
        vec![create("")],
        "out = set test1 test2",
        CommandValidation::Match("out".to_string(), "test1".to_string()),
    );
}

#[test]
fn run_multiple_arguments_first_falsy() {
    test::run_script_and_validate(
        vec![create("")],
        "out = set false or 0 or no or test",
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}

#[test]
fn run_multiple_arguments_all_falsy() {
    test::run_script_and_validate(
        vec![create("")],
        "out = set false or 0 or no",
        CommandValidation::Match("out".to_string(), "no".to_string()),
    );
}

#[test]
fn run_end_with_or() {
    test::run_script_and_error(vec![create("")], "out = set false or 0 or no or", "out");
}

#[test]
fn run_end_missing_or() {
    test::run_script_and_error(vec![create("")], "out = set false 0", "out");
}
