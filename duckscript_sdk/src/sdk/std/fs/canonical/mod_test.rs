use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = canonicalize", "out");
}

#[test]
fn run_path_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = canonicalize ./target/_duckscript",
        CommandValidation::Contains("out".to_string(), "/target/_duckscript".to_string()),
    );
}
