use super::*;
use crate::test;
use crate::test::CommandValidation;
use fsio::file::write_text_file;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_directory_path_provided() {
    test::run_script_and_error(vec![create("")], "out = mkdir", "out");
}

#[test]
fn run_directory_provided() {
    test::run_script_and_validate(
        vec![create("")],
        "out = mkdir ./target/_duckscript/mkdir/run_no_directory_provided/1/2",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_directory_already_exists() {
    test::run_script_and_validate(
        vec![create("")],
        r#"
    mkdir ./target/_duckscript/mkdir/run_no_directory_already_exists
    out = mkdir ./target/_duckscript/mkdir/run_no_directory_already_exists
    "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_directory_exists_as_file() {
    let result = write_text_file(
        "./target/_duckscript/mkdir/run_directory_exists_as_file/test.txt",
        "test file",
    );
    assert!(result.is_ok());

    test::run_script_and_error(
        vec![create("")],
        "out = mkdir ./target/_duckscript/mkdir/run_directory_exists_as_file/test.txt",
        "out",
    );
}
