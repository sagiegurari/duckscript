use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = hex_encode", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = hex_encode 255",
        CommandValidation::Match("out".to_string(), "0xff".to_string()),
    );
}
