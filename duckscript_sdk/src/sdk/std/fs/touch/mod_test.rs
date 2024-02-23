use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = touch", "out");
}

#[test]
fn run_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = touch ./target/_duckscript/touch/new/file.txt",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_path_to_existing_directory() {
    let result = fsio::directory::create("./target/_duckscript/touch/existing_dir");
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = touch ./target/_duckscript/touch/existing_dir",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
