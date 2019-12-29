use super::*;
use crate::test;
use crate::test::{CommandValidation, SetHandleCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create("")],
        "out = release",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_not_exist() {
    test::run_script_and_validate(
        vec![create("")],
        "out = release release_no_exist",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_exist() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetHandleCommand {})],
        r#"
        test_set_handle release_exist
        out = release release_exist
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
