use super::*;
use crate::sdk::std::string::bytes_to_string;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = base64_decode", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), bytes_to_string::create("")],
        r#"
handle = base64_decode aGVsbG8gd29ybGQ=
out = bytes_to_string ${handle}
"#,
        CommandValidation::Match("out".to_string(), "hello world".to_string()),
    );
}
