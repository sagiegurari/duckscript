use super::*;
use crate::sdk::std::collections::array;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = array_set", "out");
}

#[test]
fn run_only_handle() {
    test::run_script_and_error(vec![create("")], "out = array_set handle", "out");
}

#[test]
fn run_only_handle_and_index() {
    test::run_script_and_error(vec![create("")], "out = array_set handle 3", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(
        vec![create("")],
        "out = array_set bad_handle 2 value",
        "out",
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), array::create("")],
        r#"
        handle = array a b c "d e"
        out = array_set ${handle} 3 value
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_found_out_of_bounds() {
    test::run_script_and_error(
        vec![create(""), array::create("")],
        r#"
        handle = array a b c "d e"
        out = array_set ${handle} 20 value
        "#,
        "out",
    );
}
