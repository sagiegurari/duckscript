use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create("")],
        "out = exit",
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}

#[test]
fn run_0() {
    test::run_script_and_validate(
        vec![create("")],
        "out = exit 0",
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}

#[test]
fn run_positive_number() {
    test::run_script_and_validate(
        vec![create("")],
        "out = exit 10",
        CommandValidation::Match("out".to_string(), "10".to_string()),
    );
}

#[test]
fn run_negative_number() {
    test::run_script_and_error(vec![create("")], "out = exit -10", "out");
}

#[test]
fn run_text() {
    test::run_script_and_error(vec![create("")], "out = exit test", "out");
}
