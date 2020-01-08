use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = dirname", "out");
}

#[test]
fn run_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = dirname ./target/_duckscript/file.txt",
        CommandValidation::Match("out".to_string(), "./target/_duckscript".to_string()),
    );
}

#[test]
fn run_file_without_directory_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = dirname file.txt",
        CommandValidation::None,
    );
}
