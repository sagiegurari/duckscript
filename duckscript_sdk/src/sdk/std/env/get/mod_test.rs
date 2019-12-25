use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_command_and_fail(create(""), "out = get_env");
}

#[test]
fn run_env_not_exists() {
    env::remove_var("DUCKSCRIPT_SDK_GET_ENV_NOT_EXISTS");
    test::validate_command(
        create(""),
        "out = get_env DUCKSCRIPT_SDK_GET_ENV_NOT_EXISTS",
        CommandValidation::None,
    );
}

#[test]
fn run_env_exists() {
    env::set_var("DUCKSCRIPT_SDK_GET_ENV_EXISTS", "test");
    test::validate_command(
        create(""),
        "out = get_env DUCKSCRIPT_SDK_GET_ENV_EXISTS",
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
    env::remove_var("DUCKSCRIPT_SDK_GET_ENV_EXISTS");
}
