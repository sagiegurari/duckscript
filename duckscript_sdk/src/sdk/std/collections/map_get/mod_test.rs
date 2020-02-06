use super::*;
use crate::sdk::std::collections::{map, map_put};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = map_get", "out");
}

#[test]
fn run_missing_key() {
    test::run_script_and_error(vec![create("")], "out = map_get handle", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = map_get bad_handle key", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), map::create(""), map_put::create("")],
        r#"
        handle = map
        map_put ${handle} key value
        out = map_get ${handle} key
        "#,
        CommandValidation::Match("out".to_string(), "value".to_string()),
    );
}
