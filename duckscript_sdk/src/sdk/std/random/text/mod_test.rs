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
        "out = rand_text",
        CommandValidation::StringLength("out".to_string(), 1),
    );
}

#[test]
fn run_invalid_size() {
    test::run_script_and_error(vec![create("")], "out = rand_text -10", "out");
}

#[test]
fn run_valid_size() {
    test::run_script_and_validate(
        vec![create("")],
        "out = rand_text 10",
        CommandValidation::StringLength("out".to_string(), 10),
    );
}
