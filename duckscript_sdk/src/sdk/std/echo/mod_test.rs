use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    let command = create("");
    let context = test::run_command_valid_with_default_context(command, "out = echo");

    assert_eq!(context.variables.get("out").unwrap(), "0");
}

#[test]
fn run_multiple_args() {
    let command = create("");
    let context = test::run_command_valid_with_default_context(command, "out = echo 1 2 \"3 4\"");

    assert_eq!(context.variables.get("out").unwrap(), "3");
}
