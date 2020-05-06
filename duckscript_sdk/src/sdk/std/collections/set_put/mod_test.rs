use super::*;
use crate::sdk::std::collections::{set, set_size};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = set_put", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_put bad_handle 1", "out");
}

#[test]
fn run_only_set_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_put bad_handle", "out");
}

#[test]
fn run_found_no_input() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), set_size::create("")],
        r#"
        handle = set_new 1 2 3
        set_put ${handle}
        out = set_size ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "3".to_string()),
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), set_size::create("")],
        r#"
        handle = set_new 1 2 3
        set_put ${handle} 4
        out = set_size ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}

#[test]
fn run_found_duplicate() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), set_size::create("")],
        r#"
        handle = set_new 1 2 3
        set_put ${handle} 3
        out = set_size ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "3".to_string()),
    );
}
