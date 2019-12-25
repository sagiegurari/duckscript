use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::validate_command(
        create(""),
        "out = echo",
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}

#[test]
fn run_multiple_args() {
    test::validate_command(
        create(""),
        "out = echo 1 2 \"3 4\"",
        CommandValidation::Match("out".to_string(), "3".to_string()),
    );
}
