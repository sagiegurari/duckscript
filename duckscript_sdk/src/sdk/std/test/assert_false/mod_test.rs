use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_crash(vec![create("")], "out = assert_false");
}

#[test]
fn run_single_true_argument() {
    test::run_script_and_crash(vec![create("")], "out = assert_false true");
}

#[test]
fn run_single_false_argument() {
    test::run_script_and_validate(
        vec![create("")],
        "out = assert_false false",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_single_true_condition_and_message() {
    test::run_script_and_crash(vec![create("")], "out = assert_false true test error");
}
