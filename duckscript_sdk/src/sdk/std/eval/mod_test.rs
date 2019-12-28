use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(vec![create("")], "out = eval", CommandValidation::None);
}

#[test]
fn run_label_only() {
    test::run_script_and_validate(
        vec![create("")],
        "out = eval :label",
        CommandValidation::None,
    );
}

#[test]
fn run_command_with_output() {
    let set_command = SetCommand {};

    test::run_script_and_validate(
        vec![create(""), Box::new(set_command)],
        "out = eval test_set test",
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}

#[test]
fn run_command_with_no_output() {
    let set_command = SetCommand {};

    test::run_script_and_validate(
        vec![create(""), Box::new(set_command)],
        "out = eval test_set",
        CommandValidation::None,
    );
}

#[test]
fn run_command_with_spaces_in_arg() {
    let set_command = SetCommand {};

    test::run_script_and_validate(
        vec![create(""), Box::new(set_command)],
        r#"out = eval test_set "test 1 2 3""#,
        CommandValidation::Match("out".to_string(), "test 1 2 3".to_string()),
    );
}

#[test]
fn run_command_as_variable_with_output() {
    let set_command = SetCommand {};

    test::run_script_and_validate(
        vec![create(""), Box::new(set_command)],
        r#"
        command = test_set test_set
        out = eval ${command} test
        "#,
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}
