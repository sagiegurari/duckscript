use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_fail(vec![create("")], "basename");
}

#[test]
fn run_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = basename ./target/_duckscript/file.txt",
        CommandValidation::Match("out".to_string(), "file.txt".to_string()),
    );
}
