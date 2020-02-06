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
    test::run_script_and_error(vec![create("")], "out = base64_encode", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), string_to_bytes::create("")],
        r#"
handle = string_to_bytes "hello world"
out = base64_encode ${handle}
"#,
        CommandValidation::Match("out".to_string(), "aGVsbG8gd29ybGQ=".to_string()),
    );
}
