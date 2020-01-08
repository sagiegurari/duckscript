use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = alias", "out");
}

#[test]
fn run_only_name() {
    test::run_script_and_error(vec![create("")], "out = alias new", "out");
}

#[test]
fn run_valid_no_default_args() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
    alias set test_set
    out = set output
    "#,
        CommandValidation::Match("out".to_string(), "output".to_string()),
    );
}

#[test]
fn run_valid_with_default_args() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
    alias set test_set default
    out = set output
    "#,
        CommandValidation::Match("out".to_string(), "default".to_string()),
    );
}
