use super::*;
use crate::sdk::std::string::string_to_bytes;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = bytes_to_string", "out");
}

#[test]
fn run_handle_not_found() {
    test::run_script_and_error(vec![create("")], "out = bytes_to_string badhandle", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), string_to_bytes::create("")],
        r#"
handle = string_to_bytes "hello world"
out = bytes_to_string ${handle}
"#,
        CommandValidation::Match("out".to_string(), "hello world".to_string()),
    );
}
