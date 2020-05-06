use super::*;
use crate::sdk::std::collections::set;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = set_contains", "out");
}

#[test]
fn run_missing_key() {
    test::run_script_and_error(vec![create("")], "out = set_contains handle", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_contains bad_handle key", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), set::create("")],
        r#"
        handle = set_new 1 2 3
        out = set_contains ${handle} 2
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_value_not_found() {
    test::run_script_and_validate(
        vec![create(""), set::create("")],
        r#"
        handle = set_new 1 2 3
        out = set_contains ${handle} 4
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
