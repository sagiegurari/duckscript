use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_error(vec![create("")], "out = get_env", "out");
}

#[test]
fn run_env_not_exists() {
    env::remove_var("DUCKSCRIPT_SDK_GET_ENV_NOT_EXISTS");
    test::run_script_and_validate(
        vec![create("")],
        "out = get_env DUCKSCRIPT_SDK_GET_ENV_NOT_EXISTS",
        CommandValidation::None,
    );
}

#[test]
fn run_env_exists() {
    env::set_var("DUCKSCRIPT_SDK_GET_ENV_EXISTS", "test");
    test::run_script_and_validate(
        vec![create("")],
        "out = get_env DUCKSCRIPT_SDK_GET_ENV_EXISTS",
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
    env::remove_var("DUCKSCRIPT_SDK_GET_ENV_EXISTS");
}
