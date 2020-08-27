use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = is_command_defined", "out");
}

#[test]
fn run_with_empty_string() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = is_command_defined """#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_not_defined() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = is_command_defined badcommand"#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_defined() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"out = is_command_defined test_set"#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
