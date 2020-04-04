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
    test::run_script_and_error(vec![create("")], "out = array_get", "out");
}

#[test]
fn run_only_handle() {
    test::run_script_and_error(vec![create("")], "out = array_get handle", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = array_get bad_handle 2", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), array::create("")],
        r#"
        handle = array a b c "d e"
        out = array_get ${handle} 3
        "#,
        CommandValidation::Match("out".to_string(), "d e".to_string()),
    );
}

#[test]
fn run_found_out_of_bounds() {
    test::run_script_and_validate(
        vec![create(""), array::create("")],
        r#"
        handle = array a b c "d e"
        out = array_get ${handle} 20
        "#,
        CommandValidation::Undefined("out".to_string()),
    );
}
