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
    test::run_script_and_error(vec![create("")], "out = array_length", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = array_length bad_handle", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), array::create("")],
        r#"
        handle = array a b c "d e"
        out = array_length ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}
