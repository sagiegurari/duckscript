use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_command_and_fail(create(""), "set_env");
}

#[test]
fn run_single_argument() {
    test::run_command_and_fail(create(""), "set_env key");
}

#[test]
fn run_set() {
    env::remove_var("DUCKSCRIPT_SDK_SET_ENV");
    test::validate_command(
        create(""),
        "set_env DUCKSCRIPT_SDK_SET_ENV test",
        CommandValidation::None,
    );
    assert_eq!(env::var("DUCKSCRIPT_SDK_SET_ENV").unwrap(), "test");
    env::remove_var("DUCKSCRIPT_SDK_SET_ENV");
}
