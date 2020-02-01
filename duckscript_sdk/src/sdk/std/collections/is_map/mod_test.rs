use super::*;
use crate::sdk::std::collections::map;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = is_map", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_map bad_handle",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_not_array() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        handle = test_set true
        out = is_map ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), map::create("")],
        r#"
        handle = map
        out = is_map ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
