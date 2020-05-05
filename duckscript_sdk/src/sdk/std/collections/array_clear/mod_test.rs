use super::*;
use crate::sdk::std::collections::{array, array_length, array_push};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = array_clear", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = array_clear bad_handle", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![
            create(""),
            array::create(""),
            array_length::create(""),
            array_push::create(""),
        ],
        r#"
        handle = array
        array_push ${handle} 1
        array_push ${handle} 2
        array_push ${handle} 3
        array_push ${handle} 4
        array_clear ${handle}
        out = array_length ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}
