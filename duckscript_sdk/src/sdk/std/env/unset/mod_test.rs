use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_fail(vec![create("")], "unset_env");
}

#[test]
fn run_unset() {
    env::set_var("DUCKSCRIPT_SDK_UNSET_ENV", "test");
    assert!(env::var("DUCKSCRIPT_SDK_UNSET_ENV").is_ok());
    test::run_script_and_validate(
        vec![create("")],
        "unset_env DUCKSCRIPT_SDK_UNSET_ENV",
        CommandValidation::None,
    );
    assert!(env::var("DUCKSCRIPT_SDK_UNSET_ENV").is_err());
}
