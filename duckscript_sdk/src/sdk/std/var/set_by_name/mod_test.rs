use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_error(vec![create("")], "out = set_by_name", "out");
}

#[test]
fn run_only_name() {
    test::run_script_and_error(vec![create("")], "out = set_by_name test", "out");
}

#[test]
fn run_valid() {
    let context = test::run_script_and_validate(
        vec![create("")],
        "out = set_by_name test value",
        CommandValidation::Match("out".to_string(), "value".to_string()),
    );

    assert_eq!(context.variables.get("test").unwrap(), "value");
}
