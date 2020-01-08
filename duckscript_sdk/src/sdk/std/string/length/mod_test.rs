use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = length", "out");
}

#[test]
fn run_empty_text() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = length """#,
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}

#[test]
fn run_text() {
    test::run_script_and_validate(
        vec![create("")],
        "out = length text",
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}
