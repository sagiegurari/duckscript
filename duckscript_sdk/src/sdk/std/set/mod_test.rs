use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    let command = create("");
    let context = test::run_command_valid_with_default_context(command, "out = set");

    assert!(context.variables.is_empty());
}

#[test]
fn run_single_argument() {
    let command = create("");
    let context = test::run_command_valid_with_default_context(command, "out = set test");

    assert_eq!(context.variables.get("out").unwrap(), "test");
}

#[test]
fn run_multiple_arguments() {
    let command = create("");
    let context = test::run_command_valid_with_default_context(command, "out = set test1 test2");

    assert_eq!(context.variables.get("out").unwrap(), "test1");
}
