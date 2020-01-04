use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_empty",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_with_empty_string() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = is_empty """#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_with_text() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = is_empty a"#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
