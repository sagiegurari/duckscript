use super::*;
use crate::sdk::std::collections::{array_length, set};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = set_to_array", "out");
}
#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_to_array bad_handle", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), set::create(""), array_length::create("")],
        r#"
        handle = set_new 1 2 3
        array = set_to_array ${handle}
        out = array_length ${array}
        "#,
        CommandValidation::Match("out".to_string(), "3".to_string()),
    );
}
