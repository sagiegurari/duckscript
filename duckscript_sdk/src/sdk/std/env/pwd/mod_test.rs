use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = pwd",
        CommandValidation::Contains("out".to_string(), "duckscript".to_string()),
    );
}
