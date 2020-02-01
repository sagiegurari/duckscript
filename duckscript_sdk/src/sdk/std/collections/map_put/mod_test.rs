use super::*;
use crate::sdk::std::collections::{map, map_get};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = map_put", "out");
}

#[test]
fn run_missing_key() {
    test::run_script_and_error(vec![create("")], "out = map_put handle", "out");
}

#[test]
fn run_missing_value() {
    test::run_script_and_error(vec![create("")], "out = map_put handle key", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(
        vec![create("")],
        "out = map_put bad_handle key value",
        "out",
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), map::create(""), map_get::create("")],
        r#"
        handle = map
        map_put ${handle} key value 
        out = map_get ${handle} key
        "#,
        CommandValidation::Match("out".to_string(), "value".to_string()),
    );
}
