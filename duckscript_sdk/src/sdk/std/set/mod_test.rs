use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::validate_command(create(""), "out = set", CommandValidation::None);
}

#[test]
fn run_single_argument() {
    test::validate_command(
        create(""),
        "out = set test",
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}

#[test]
fn run_multiple_arguments() {
    test::validate_command(
        create(""),
        "out = set test1 test2",
        CommandValidation::Match("out".to_string(), "test1".to_string()),
    );
}
