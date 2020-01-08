use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = calc", "out");
}

#[test]
fn run_operation() {
    test::run_script_and_validate(
        vec![create("")],
        "out = calc 1 + 5 * 7",
        CommandValidation::Match("out".to_string(), "36".to_string()),
    );
}
