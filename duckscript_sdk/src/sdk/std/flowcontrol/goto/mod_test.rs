use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_error(vec![create("")], "out = goto", "out");
}

#[test]
fn run_mutliple_arguments() {
    test::run_script_and_error(vec![create("")], "out = goto :label1 :label2", "out");
}

#[test]
fn run_not_label_format() {
    test::run_script_and_error(vec![create("")], "out = goto label", "out");
}

#[test]
fn run_label_not_found() {
    test::run_script_and_crash(vec![create("")], "goto :unknown");
}

#[test]
fn run_no_args() {
    let set_command = SetCommand {};

    let context = test::run_script_and_validate(
        vec![create(""), Box::new(set_command)],
        r#"
goto :valid

out1 = test_set 1

:valid out2 = test_set 2
        "#,
        CommandValidation::Match("out2".to_string(), "2".to_string()),
    );

    assert_eq!(context.variables.len(), 1);
}
