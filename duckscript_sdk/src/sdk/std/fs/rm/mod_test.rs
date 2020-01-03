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
    test::run_script_and_fail(vec![create("")], "rm");
}

#[test]
fn run_no_path_provided_only_flags() {
    test::run_script_and_fail(vec![create("")], "rm -r");
}

#[test]
fn run_path_not_exists() {
    test::run_script_and_validate(
        vec![create("")],
        "out = rm ./target/_duckscript/rm/newdir",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_path_not_empty_not_recursive() {
    let result = io::create_directory("./target/_duckscript/rm/not_empty/dir1");
    assert!(result.is_ok());

    test::run_script_and_fail(vec![create("")], "rm ./target/_duckscript/rm/not_empty");
}

#[test]
fn run_path_is_file() {
    let path = Path::new("./target/_duckscript/rm/file.txt");
    let result = io::create_empty_file("./target/_duckscript/rm/file.txt");
    assert!(result.is_ok());
    assert!(path.exists());

    test::run_script_and_validate(
        vec![create("")],
        "out = rm ./target/_duckscript/rm/file.txt",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );

    assert!(!path.exists());
}

#[test]
fn run_path_recursive() {
    let path = Path::new("./target/_duckscript/rm/recursive/file.txt");
    let result = io::create_empty_file("./target/_duckscript/rm/recursive/file.txt");
    assert!(result.is_ok());
    assert!(path.exists());

    test::run_script_and_validate(
        vec![create("")],
        "out = rm -r ./target/_duckscript/rm/recursive",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );

    assert!(!path.exists());
}
