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
    test::run_script_and_error(vec![create("")], "out = set_size", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_size bad_handle", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), set_put::create("")],
        r#"
        handle = set_new
        set_put ${handle} 1
        set_put ${handle} 2
        set_put ${handle} 3
        set_put ${handle} 1
        out = set_size ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "3".to_string()),
    );
}
