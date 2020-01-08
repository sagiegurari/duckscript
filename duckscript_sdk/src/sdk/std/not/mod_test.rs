use super::*;
use crate::test;
use crate::test::{CommandValidation, ErrorCommand, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = not", "out");
}

#[test]
fn run_single_value_true() {
    test::run_script_and_validate(
        vec![create("")],
        "out = not true",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_single_value_false() {
    test::run_script_and_validate(
        vec![create("")],
        "out = not false",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_command_value_true() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        "out = not test_set true",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_command_value_false() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        "out = not test_set false",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_command_no_value() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        "out = not test_set",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_command_error() {
    test::run_script_and_error(
        vec![create(""), Box::new(ErrorCommand {})],
        "out = not test_error",
        "out",
    );
}
