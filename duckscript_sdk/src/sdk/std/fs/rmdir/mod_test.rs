use super::*;
use crate::test;
use crate::test::CommandValidation;
use crate::utils::io;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_fail(vec![create("")], "rmdir");
}

#[test]
fn run_path_not_exists() {
    test::run_script_and_validate(
        vec![create("")],
        "out = rmdir ./target/_duckscript/rmdir/newdir",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_path_not_empty() {
    let result = io::create_directory("./target/_duckscript/rmdir/not_empty/dir1");
    assert!(result.is_ok());

    test::run_script_and_fail(
        vec![create("")],
        "rmdir ./target/_duckscript/rmdir/not_empty",
    );
}

#[test]
fn run_path_is_file() {
    let result = io::create_empty_file("./target/_duckscript/rmdir/file.txt");
    assert!(result.is_ok());

    test::run_script_and_fail(
        vec![create("")],
        "rmdir ./target/_duckscript/rmdir/file.txt",
    );
}

#[test]
fn run_valid() {
    let result = io::create_directory("./target/_duckscript/rmdir/existing_dir");
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = rmdir ./target/_duckscript/rmdir/existing_dir",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
