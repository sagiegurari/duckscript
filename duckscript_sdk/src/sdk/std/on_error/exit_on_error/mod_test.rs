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
        "out = exit_on_error",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_true() {
    test::run_script_and_validate(
        vec![create("")],
        "out = exit_on_error true",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_false() {
    test::run_script_and_validate(
        vec![create("")],
        "out = exit_on_error false",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_true_and_check() {
    test::run_script_and_validate(
        vec![create("")],
        r#"
        exit_on_error true
        out = exit_on_error
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_false_and_check() {
    test::run_script_and_validate(
        vec![create("")],
        r#"
        exit_on_error false
        out = exit_on_error
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
