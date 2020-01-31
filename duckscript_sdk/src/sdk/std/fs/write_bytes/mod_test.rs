use super::*;
use crate::sdk::std::string::string_to_bytes;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("")], "out = write_binary_file", "out");
}

#[test]
fn run_no_text_provided() {
    test::run_script_and_error(
        vec![create("")],
        "out = write_binary_file ./target/_duckscript/write/write_binary_file.txt",
        "out",
    );
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), string_to_bytes::create("")],
        r#"
handle = string_to_bytes "line 1\nline 2"
out = write_binary_file ./target/_duckscript/write/write_binary_file.txt ${handle}
"#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
    let text = io::read_text_file("./target/_duckscript/write/write_binary_file.txt").unwrap();
    assert_eq!(text, "line 1\nline 2")
}
