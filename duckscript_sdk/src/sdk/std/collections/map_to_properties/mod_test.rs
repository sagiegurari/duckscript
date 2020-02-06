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
    test::run_script_and_error(vec![create("")], "out = map_to_properties", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(
        vec![create("")],
        "out = map_to_properties bad_handle",
        "out",
    );
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), map::create(""), map_put::create("")],
        r#"
        handle = map
        map_put ${handle} a 1
        out = map_to_properties ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "a=1".to_string()),
    );
}

#[test]
fn run_with_prefix() {
    test::run_script_and_validate(
        vec![create(""), map::create(""), map_put::create("")],
        r#"
        handle = map
        map_put ${handle} a 1
        out = map_to_properties --prefix config ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "config.a=1".to_string()),
    );
}
