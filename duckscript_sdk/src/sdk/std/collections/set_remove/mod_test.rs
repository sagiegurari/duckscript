use super::*;
use crate::sdk::std::collections::{set, set_put};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = set_remove", "out");
}

#[test]
fn run_missing_key() {
    test::run_script_and_error(vec![create("")], "out = set_remove handle", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_remove bad_handle key", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), set_put::create("")],
        r#"
        handle = set_new
        set_put ${handle} value
        out = set_remove ${handle} value
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_twice() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), set_put::create("")],
        r#"
        handle = set_new
        set_put ${handle} value
        out = set_remove ${handle} value
        out = set_remove ${handle} value
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
