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
    test::run_script_and_fail(vec![create("")], "cp");
}

#[test]
fn run_single_input() {
    test::run_script_and_fail(vec![create("")], "cp a");
}

#[test]
fn run_path_not_exists() {
    test::run_script_and_validate(
        vec![create("")],
        "out = cp ./target/_duckscript/cp/newdir1 ./target/_duckscript/cp/newdir2",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_copy_file() {
    let result = io::write_text_file("./target/_duckscript/cp/file/file1.txt", "test");
    assert!(result.is_ok());
    let mut path = Path::new("./target/_duckscript/cp/file/file1.txt");
    assert!(path.exists());

    test::run_script_and_validate(
        vec![create("")],
        "out = cp ./target/_duckscript/cp/file/file1.txt ./target/_duckscript/cp/file/file2.txt",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );

    assert!(path.exists());
    path = Path::new("./target/_duckscript/cp/file/file2.txt");
    assert!(path.exists());

    let text = io::read_text_file("./target/_duckscript/cp/file/file2.txt");
    assert_eq!(text.unwrap(), "test");
}
