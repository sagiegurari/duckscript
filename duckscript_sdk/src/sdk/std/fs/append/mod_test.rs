use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("")], "out = appendfile", "out");
}

#[test]
fn run_no_text_provided() {
    test::run_script_and_error(
        vec![create("")],
        "out = appendfile ./target/_duckscript/append/appendfile.txt",
        "out",
    );
}

#[test]
fn run_not_exists() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = appendfile ./target/_duckscript/append/appendfile.txt "line 1\nline 2""#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
    let text = io::read_text_file("./target/_duckscript/append/appendfile.txt").unwrap();
    assert_eq!(text, "line 1\nline 2")
}
