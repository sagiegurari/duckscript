use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_extension_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = temp_file",
        CommandValidation::Contains("out".to_string(), ".tmp".to_string()),
    );
}

#[test]
fn run_extension_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = temp_file toml",
        CommandValidation::Contains("out".to_string(), ".toml".to_string()),
    );
}
