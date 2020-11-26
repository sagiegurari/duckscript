use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = spawn", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = spawn echo test",
        CommandValidation::PositiveNumber("out".to_string()),
    );
}

#[test]
fn run_valid_silent() {
    test::run_script_and_validate(
        vec![create("")],
        "out = spawn --silent echo test",
        CommandValidation::PositiveNumber("out".to_string()),
    );
}
