use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::validate_command(create(""), "out = set", None, None, false);
}

#[test]
fn run_single_argument() {
    test::validate_command(
        create(""),
        "out = set test",
        Some("out".to_string()),
        Some("test".to_string()),
        false,
    );
}

#[test]
fn run_multiple_arguments() {
    test::validate_command(
        create(""),
        "out = set test1 test2",
        Some("out".to_string()),
        Some("test1".to_string()),
        false,
    );
}
