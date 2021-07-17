use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = kebabcase", "out");
}

#[test]
fn run_single_argument() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = kebabcase "Hello, World!""#,
        CommandValidation::Match("out".to_string(), "hello-world".to_string()),
    );
}
