use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_fail(vec![create("")], "out = is_defined");
}

#[test]
fn run_with_empty_string() {
    test::run_script_and_fail(vec![create("")], r#"out = is_defined """#);
}

#[test]
fn run_not_defined() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = is_defined test_var"#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_defined() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        test_var = test_set a
        out = is_defined test_var
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
