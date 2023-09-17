use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_error(vec![create("")], "out = set_env", "out");
}

#[test]
fn run_single_argument() {
    test::run_script_and_error(vec![create("")], "out = set_env key", "out");
}

#[test]
fn run_empty_var_name() {
    test::run_script_and_error(vec![create("")], "out = set_env \"\" value", "out");
}

#[test]
fn run_set() {
    env::remove_var("DUCKSCRIPT_SDK_SET_ENV");
    test::run_script_and_validate(
        vec![create("")],
        "out = set_env DUCKSCRIPT_SDK_SET_ENV test",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
    assert_eq!(env::var("DUCKSCRIPT_SDK_SET_ENV").unwrap(), "test");
    env::remove_var("DUCKSCRIPT_SDK_SET_ENV");
}
