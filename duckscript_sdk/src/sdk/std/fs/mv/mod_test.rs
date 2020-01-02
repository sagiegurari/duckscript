use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_fail(vec![create("")], "mv");
}

#[test]
fn run_single_path_provided() {
    test::run_script_and_fail(vec![create("")], "mv a");
}

#[test]
fn run_input_path_not_exists() {
    test::run_script_and_fail(
        vec![create("")],
        "mv ./target/_duckscript/mv/not_exists.txt ./target/_duckscript/mv/not_exists/",
    );
}

#[test]
fn run_file_to_file() {
    let mut path = Path::new("./target/_duckscript/mv/run_file_to_file/1/file1.txt");
    let result = io::create_empty_file("./target/_duckscript/mv/run_file_to_file/1/file1.txt");
    assert!(result.is_ok());
    assert!(path.exists());

    test::run_script_and_validate(vec![create("")], r#"
    out = mv ./target/_duckscript/mv/run_file_to_file/1/file1.txt ./target/_duckscript/mv/run_file_to_file/2/file2.txt
    "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),);

    assert!(!path.exists());
    path = Path::new("./target/_duckscript/mv/run_file_to_file/2/file2.txt");
    assert!(path.exists());
}

#[test]
fn run_file_to_directory() {
    let mut path = Path::new("./target/_duckscript/mv/run_file_to_directory/1/file1.txt");
    let result = io::create_empty_file("./target/_duckscript/mv/run_file_to_directory/1/file1.txt");
    assert!(result.is_ok());
    assert!(path.exists());

    test::run_script_and_validate(vec![create("")], r#"
    out = mv ./target/_duckscript/mv/run_file_to_directory/1/file1.txt ./target/_duckscript/mv/run_file_to_directory/2
    "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),);

    assert!(!path.exists());
    path = Path::new("./target/_duckscript/mv/run_file_to_directory/2/file1.txt");
    assert!(path.exists());
}

#[test]
fn run_directory_to_directory() {
    let mut path = Path::new("./target/_duckscript/mv/run_directory_to_directory/1/1/file1.txt");
    let result =
        io::create_empty_file("./target/_duckscript/mv/run_directory_to_directory/1/1/file1.txt");
    assert!(result.is_ok());
    assert!(path.exists());

    test::run_script_and_validate(vec![create("")], r#"
    out = mv ./target/_duckscript/mv/run_directory_to_directory/1 ./target/_duckscript/mv/run_directory_to_directory/2
    "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),);

    assert!(!path.exists());
    path = Path::new("./target/_duckscript/mv/run_directory_to_directory/2/1/1/file1.txt");
    assert!(path.exists());
}
