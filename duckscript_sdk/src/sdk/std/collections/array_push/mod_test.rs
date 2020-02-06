use super::*;
use crate::sdk::std::collections::{array, array_pop};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = array_push", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = array_push bad_handle 1", "out");
}

#[test]
fn run_only_array_not_found() {
    test::run_script_and_error(vec![create("")], "out = array_push bad_handle", "out");
}

#[test]
fn run_found_no_input() {
    test::run_script_and_validate(
        vec![create(""), array::create(""), array_pop::create("")],
        r#"
        handle = array 1 2 3
        array_push ${handle}
        out = array_pop ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "3".to_string()),
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), array::create(""), array_pop::create("")],
        r#"
        handle = array 1 2 3
        array_push ${handle} 4
        out = array_pop ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}
