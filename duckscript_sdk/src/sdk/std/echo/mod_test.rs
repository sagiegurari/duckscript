use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::validate_command(
        create(""),
        "out = echo",
        Some("out".to_string()),
        Some("0".to_string()),
        false,
    );
}

#[test]
fn run_multiple_args() {
    test::validate_command(
        create(""),
        "out = echo 1 2 \"3 4\"",
        Some("out".to_string()),
        Some("3".to_string()),
        false,
    );
}
