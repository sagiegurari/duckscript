use super::*;
use crate::test;
use crate::test::CommandValidation;
use fsio;
use fsio::file::ensure_exists;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = rmdir", "out");
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
    let result = fsio::directory::create("./target/_duckscript/rmdir/not_empty/dir1");
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = rmdir ./target/_duckscript/rmdir/not_empty",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_path_is_file() {
    let result = ensure_exists("./target/_duckscript/rmdir/file.txt");
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = rmdir ./target/_duckscript/rmdir/file.txt",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_valid() {
    let result = fsio::directory::create("./target/_duckscript/rmdir/existing_dir");
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = rmdir ./target/_duckscript/rmdir/existing_dir",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
