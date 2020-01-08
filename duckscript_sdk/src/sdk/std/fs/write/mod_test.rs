use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("")], "out = writefile", "out");
}

#[test]
fn run_no_text_provided() {
    test::run_script_and_error(
        vec![create("")],
        "out = writefile ./target/_duckscript/write/writefile.txt",
        "out",
    );
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = writefile ./target/_duckscript/write/writefile.txt "line 1\nline 2""#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
    let text = io::read_text_file("./target/_duckscript/write/writefile.txt").unwrap();
    assert_eq!(text, "line 1\nline 2")
}
